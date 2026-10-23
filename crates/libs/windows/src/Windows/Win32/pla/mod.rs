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
windows_core::imp::interface_hierarchy!(IAlertDataCollector, windows_core::IUnknown, super::IDispatch, IDataCollector);
#[cfg(feature = "oaidl")]
impl IAlertDataCollector {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_AlertThresholds(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AlertThresholds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_AlertThresholds(&self, alerts: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AlertThresholds)(windows_core::Interface::as_raw(self), alerts) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_EventLog(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EventLog)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_EventLog(&self, log: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_EventLog)(windows_core::Interface::as_raw(self), log) }
    }
    pub unsafe fn get_SampleInterval(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SampleInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_SampleInterval(&self, interval: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SampleInterval)(windows_core::Interface::as_raw(self), interval) }
    }
    pub unsafe fn get_Task(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Task)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Task(&self, task: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Task)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(task)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_TaskRunAsSelf(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TaskRunAsSelf)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_TaskRunAsSelf(&self, runasself: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TaskRunAsSelf)(windows_core::Interface::as_raw(self), runasself) }
    }
    pub unsafe fn get_TaskArguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TaskArguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_TaskArguments(&self, task: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TaskArguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(task)) }
    }
    pub unsafe fn get_TaskUserTextArguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TaskUserTextArguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_TaskUserTextArguments(&self, task: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TaskUserTextArguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(task)) }
    }
    pub unsafe fn get_TriggerDataCollectorSet(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TriggerDataCollectorSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_TriggerDataCollectorSet(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TriggerDataCollectorSet)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAlertDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_AlertThresholds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_AlertThresholds: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_AlertThresholds: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_AlertThresholds: usize,
    #[cfg(feature = "wtypes")]
    pub get_EventLog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_EventLog: usize,
    #[cfg(feature = "wtypes")]
    pub put_EventLog: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_EventLog: usize,
    pub get_SampleInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_SampleInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_Task: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Task: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_TaskRunAsSelf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_TaskRunAsSelf: usize,
    #[cfg(feature = "wtypes")]
    pub put_TaskRunAsSelf: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_TaskRunAsSelf: usize,
    pub get_TaskArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_TaskArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_TaskUserTextArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_TaskUserTextArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_TriggerDataCollectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_TriggerDataCollectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAlertDataCollector_Impl: IDataCollector_Impl {
    fn get_AlertThresholds(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_AlertThresholds(&self, alerts: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_EventLog(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_EventLog(&self, log: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_SampleInterval(&self) -> windows_core::Result<u32>;
    fn put_SampleInterval(&self, interval: u32) -> windows_core::Result<()>;
    fn get_Task(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Task(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_TaskRunAsSelf(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_TaskRunAsSelf(&self, runasself: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_TaskArguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_TaskArguments(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_TaskUserTextArguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_TaskUserTextArguments(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_TriggerDataCollectorSet(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_TriggerDataCollectorSet(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAlertDataCollector_Vtbl {
    pub const fn new<Identity: IAlertDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AlertThresholds<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alerts: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::get_AlertThresholds(this) {
                    Ok(ok__) => {
                        alerts.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AlertThresholds<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alerts: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::put_AlertThresholds(this, core::mem::transmute_copy(&alerts)).into()
            }
        }
        unsafe extern "system" fn get_EventLog<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, log: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::get_EventLog(this) {
                    Ok(ok__) => {
                        log.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_EventLog<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, log: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::put_EventLog(this, core::mem::transmute_copy(&log)).into()
            }
        }
        unsafe extern "system" fn get_SampleInterval<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::get_SampleInterval(this) {
                    Ok(ok__) => {
                        interval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SampleInterval<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::put_SampleInterval(this, core::mem::transmute_copy(&interval)).into()
            }
        }
        unsafe extern "system" fn get_Task<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::get_Task(this) {
                    Ok(ok__) => {
                        task.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Task<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::put_Task(this, core::mem::transmute(&task)).into()
            }
        }
        unsafe extern "system" fn get_TaskRunAsSelf<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runasself: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::get_TaskRunAsSelf(this) {
                    Ok(ok__) => {
                        runasself.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TaskRunAsSelf<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runasself: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::put_TaskRunAsSelf(this, core::mem::transmute_copy(&runasself)).into()
            }
        }
        unsafe extern "system" fn get_TaskArguments<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::get_TaskArguments(this) {
                    Ok(ok__) => {
                        task.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TaskArguments<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::put_TaskArguments(this, core::mem::transmute(&task)).into()
            }
        }
        unsafe extern "system" fn get_TaskUserTextArguments<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::get_TaskUserTextArguments(this) {
                    Ok(ok__) => {
                        task.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TaskUserTextArguments<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::put_TaskUserTextArguments(this, core::mem::transmute(&task)).into()
            }
        }
        unsafe extern "system" fn get_TriggerDataCollectorSet<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::get_TriggerDataCollectorSet(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TriggerDataCollectorSet<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::put_TriggerDataCollectorSet(this, core::mem::transmute(&name)).into()
            }
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            get_AlertThresholds: get_AlertThresholds::<Identity, OFFSET>,
            put_AlertThresholds: put_AlertThresholds::<Identity, OFFSET>,
            get_EventLog: get_EventLog::<Identity, OFFSET>,
            put_EventLog: put_EventLog::<Identity, OFFSET>,
            get_SampleInterval: get_SampleInterval::<Identity, OFFSET>,
            put_SampleInterval: put_SampleInterval::<Identity, OFFSET>,
            get_Task: get_Task::<Identity, OFFSET>,
            put_Task: put_Task::<Identity, OFFSET>,
            get_TaskRunAsSelf: get_TaskRunAsSelf::<Identity, OFFSET>,
            put_TaskRunAsSelf: put_TaskRunAsSelf::<Identity, OFFSET>,
            get_TaskArguments: get_TaskArguments::<Identity, OFFSET>,
            put_TaskArguments: put_TaskArguments::<Identity, OFFSET>,
            get_TaskUserTextArguments: get_TaskUserTextArguments::<Identity, OFFSET>,
            put_TaskUserTextArguments: put_TaskUserTextArguments::<Identity, OFFSET>,
            get_TriggerDataCollectorSet: get_TriggerDataCollectorSet::<Identity, OFFSET>,
            put_TriggerDataCollectorSet: put_TriggerDataCollectorSet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAlertDataCollector as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IApiTracingDataCollector, windows_core::IUnknown, super::IDispatch, IDataCollector);
#[cfg(feature = "oaidl")]
impl IApiTracingDataCollector {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_LogApiNamesOnly(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LogApiNamesOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_LogApiNamesOnly(&self, logapinames: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LogApiNamesOnly)(windows_core::Interface::as_raw(self), logapinames) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_LogApisRecursively(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LogApisRecursively)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_LogApisRecursively(&self, logrecursively: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LogApisRecursively)(windows_core::Interface::as_raw(self), logrecursively) }
    }
    pub unsafe fn get_ExePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ExePath(&self, exepath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ExePath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(exepath)) }
    }
    pub unsafe fn get_LogFilePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LogFilePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_LogFilePath(&self, logfilepath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LogFilePath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(logfilepath)) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_IncludeModules(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IncludeModules)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_IncludeModules(&self, includemodules: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IncludeModules)(windows_core::Interface::as_raw(self), includemodules) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_IncludeApis(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IncludeApis)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_IncludeApis(&self, includeapis: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IncludeApis)(windows_core::Interface::as_raw(self), includeapis) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_ExcludeApis(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExcludeApis)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_ExcludeApis(&self, excludeapis: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ExcludeApis)(windows_core::Interface::as_raw(self), excludeapis) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IApiTracingDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_LogApiNamesOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_LogApiNamesOnly: usize,
    #[cfg(feature = "wtypes")]
    pub put_LogApiNamesOnly: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_LogApiNamesOnly: usize,
    #[cfg(feature = "wtypes")]
    pub get_LogApisRecursively: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_LogApisRecursively: usize,
    #[cfg(feature = "wtypes")]
    pub put_LogApisRecursively: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_LogApisRecursively: usize,
    pub get_ExePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ExePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_LogFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_LogFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_IncludeModules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_IncludeModules: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_IncludeModules: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_IncludeModules: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_IncludeApis: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_IncludeApis: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_IncludeApis: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_IncludeApis: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_ExcludeApis: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_ExcludeApis: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_ExcludeApis: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_ExcludeApis: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IApiTracingDataCollector_Impl: IDataCollector_Impl {
    fn get_LogApiNamesOnly(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_LogApiNamesOnly(&self, logapinames: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_LogApisRecursively(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_LogApisRecursively(&self, logrecursively: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_ExePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ExePath(&self, exepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_LogFilePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_LogFilePath(&self, logfilepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_IncludeModules(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_IncludeModules(&self, includemodules: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_IncludeApis(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_IncludeApis(&self, includeapis: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_ExcludeApis(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_ExcludeApis(&self, excludeapis: *const super::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IApiTracingDataCollector_Vtbl {
    pub const fn new<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_LogApiNamesOnly<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logapinames: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::get_LogApiNamesOnly(this) {
                    Ok(ok__) => {
                        logapinames.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LogApiNamesOnly<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logapinames: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::put_LogApiNamesOnly(this, core::mem::transmute_copy(&logapinames)).into()
            }
        }
        unsafe extern "system" fn get_LogApisRecursively<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logrecursively: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::get_LogApisRecursively(this) {
                    Ok(ok__) => {
                        logrecursively.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LogApisRecursively<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logrecursively: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::put_LogApisRecursively(this, core::mem::transmute_copy(&logrecursively)).into()
            }
        }
        unsafe extern "system" fn get_ExePath<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, exepath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::get_ExePath(this) {
                    Ok(ok__) => {
                        exepath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ExePath<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, exepath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::put_ExePath(this, core::mem::transmute(&exepath)).into()
            }
        }
        unsafe extern "system" fn get_LogFilePath<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfilepath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::get_LogFilePath(this) {
                    Ok(ok__) => {
                        logfilepath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LogFilePath<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfilepath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::put_LogFilePath(this, core::mem::transmute(&logfilepath)).into()
            }
        }
        unsafe extern "system" fn get_IncludeModules<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includemodules: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::get_IncludeModules(this) {
                    Ok(ok__) => {
                        includemodules.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IncludeModules<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includemodules: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::put_IncludeModules(this, core::mem::transmute_copy(&includemodules)).into()
            }
        }
        unsafe extern "system" fn get_IncludeApis<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includeapis: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::get_IncludeApis(this) {
                    Ok(ok__) => {
                        includeapis.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IncludeApis<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includeapis: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::put_IncludeApis(this, core::mem::transmute_copy(&includeapis)).into()
            }
        }
        unsafe extern "system" fn get_ExcludeApis<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, excludeapis: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::get_ExcludeApis(this) {
                    Ok(ok__) => {
                        excludeapis.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ExcludeApis<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, excludeapis: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::put_ExcludeApis(this, core::mem::transmute_copy(&excludeapis)).into()
            }
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            get_LogApiNamesOnly: get_LogApiNamesOnly::<Identity, OFFSET>,
            put_LogApiNamesOnly: put_LogApiNamesOnly::<Identity, OFFSET>,
            get_LogApisRecursively: get_LogApisRecursively::<Identity, OFFSET>,
            put_LogApisRecursively: put_LogApisRecursively::<Identity, OFFSET>,
            get_ExePath: get_ExePath::<Identity, OFFSET>,
            put_ExePath: put_ExePath::<Identity, OFFSET>,
            get_LogFilePath: get_LogFilePath::<Identity, OFFSET>,
            put_LogFilePath: put_LogFilePath::<Identity, OFFSET>,
            get_IncludeModules: get_IncludeModules::<Identity, OFFSET>,
            put_IncludeModules: put_IncludeModules::<Identity, OFFSET>,
            get_IncludeApis: get_IncludeApis::<Identity, OFFSET>,
            put_IncludeApis: put_IncludeApis::<Identity, OFFSET>,
            get_ExcludeApis: get_ExcludeApis::<Identity, OFFSET>,
            put_ExcludeApis: put_ExcludeApis::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApiTracingDataCollector as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IConfigurationDataCollector, windows_core::IUnknown, super::IDispatch, IDataCollector);
#[cfg(feature = "oaidl")]
impl IConfigurationDataCollector {
    pub unsafe fn get_FileMaxCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FileMaxCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_FileMaxCount(&self, count: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FileMaxCount)(windows_core::Interface::as_raw(self), count) }
    }
    pub unsafe fn get_FileMaxRecursiveDepth(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FileMaxRecursiveDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_FileMaxRecursiveDepth(&self, depth: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FileMaxRecursiveDepth)(windows_core::Interface::as_raw(self), depth) }
    }
    pub unsafe fn get_FileMaxTotalSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FileMaxTotalSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_FileMaxTotalSize(&self, size: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FileMaxTotalSize)(windows_core::Interface::as_raw(self), size) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_Files(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Files)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_Files(&self, files: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Files)(windows_core::Interface::as_raw(self), files) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_ManagementQueries(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ManagementQueries)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_ManagementQueries(&self, queries: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ManagementQueries)(windows_core::Interface::as_raw(self), queries) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_QueryNetworkAdapters(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_QueryNetworkAdapters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_QueryNetworkAdapters(&self, network: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_QueryNetworkAdapters)(windows_core::Interface::as_raw(self), network) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_RegistryKeys(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RegistryKeys)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_RegistryKeys(&self, query: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RegistryKeys)(windows_core::Interface::as_raw(self), query) }
    }
    pub unsafe fn get_RegistryMaxRecursiveDepth(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RegistryMaxRecursiveDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_RegistryMaxRecursiveDepth(&self, depth: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RegistryMaxRecursiveDepth)(windows_core::Interface::as_raw(self), depth) }
    }
    pub unsafe fn get_SystemStateFile(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SystemStateFile)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_SystemStateFile(&self, filename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SystemStateFile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filename)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IConfigurationDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    pub get_FileMaxCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_FileMaxCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_FileMaxRecursiveDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_FileMaxRecursiveDepth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_FileMaxTotalSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_FileMaxTotalSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_Files: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_Files: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_Files: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_Files: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_ManagementQueries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_ManagementQueries: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_ManagementQueries: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_ManagementQueries: usize,
    #[cfg(feature = "wtypes")]
    pub get_QueryNetworkAdapters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_QueryNetworkAdapters: usize,
    #[cfg(feature = "wtypes")]
    pub put_QueryNetworkAdapters: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_QueryNetworkAdapters: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_RegistryKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_RegistryKeys: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_RegistryKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_RegistryKeys: usize,
    pub get_RegistryMaxRecursiveDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_RegistryMaxRecursiveDepth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_SystemStateFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_SystemStateFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IConfigurationDataCollector_Impl: IDataCollector_Impl {
    fn get_FileMaxCount(&self) -> windows_core::Result<u32>;
    fn put_FileMaxCount(&self, count: u32) -> windows_core::Result<()>;
    fn get_FileMaxRecursiveDepth(&self) -> windows_core::Result<u32>;
    fn put_FileMaxRecursiveDepth(&self, depth: u32) -> windows_core::Result<()>;
    fn get_FileMaxTotalSize(&self) -> windows_core::Result<u32>;
    fn put_FileMaxTotalSize(&self, size: u32) -> windows_core::Result<()>;
    fn get_Files(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_Files(&self, files: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_ManagementQueries(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_ManagementQueries(&self, queries: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_QueryNetworkAdapters(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_QueryNetworkAdapters(&self, network: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_RegistryKeys(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_RegistryKeys(&self, query: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_RegistryMaxRecursiveDepth(&self) -> windows_core::Result<u32>;
    fn put_RegistryMaxRecursiveDepth(&self, depth: u32) -> windows_core::Result<()>;
    fn get_SystemStateFile(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_SystemStateFile(&self, filename: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IConfigurationDataCollector_Vtbl {
    pub const fn new<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_FileMaxCount<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::get_FileMaxCount(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FileMaxCount<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::put_FileMaxCount(this, core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn get_FileMaxRecursiveDepth<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::get_FileMaxRecursiveDepth(this) {
                    Ok(ok__) => {
                        depth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FileMaxRecursiveDepth<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::put_FileMaxRecursiveDepth(this, core::mem::transmute_copy(&depth)).into()
            }
        }
        unsafe extern "system" fn get_FileMaxTotalSize<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::get_FileMaxTotalSize(this) {
                    Ok(ok__) => {
                        size.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FileMaxTotalSize<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::put_FileMaxTotalSize(this, core::mem::transmute_copy(&size)).into()
            }
        }
        unsafe extern "system" fn get_Files<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, files: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::get_Files(this) {
                    Ok(ok__) => {
                        files.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Files<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, files: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::put_Files(this, core::mem::transmute_copy(&files)).into()
            }
        }
        unsafe extern "system" fn get_ManagementQueries<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queries: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::get_ManagementQueries(this) {
                    Ok(ok__) => {
                        queries.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ManagementQueries<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queries: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::put_ManagementQueries(this, core::mem::transmute_copy(&queries)).into()
            }
        }
        unsafe extern "system" fn get_QueryNetworkAdapters<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, network: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::get_QueryNetworkAdapters(this) {
                    Ok(ok__) => {
                        network.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_QueryNetworkAdapters<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, network: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::put_QueryNetworkAdapters(this, core::mem::transmute_copy(&network)).into()
            }
        }
        unsafe extern "system" fn get_RegistryKeys<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::get_RegistryKeys(this) {
                    Ok(ok__) => {
                        query.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RegistryKeys<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::put_RegistryKeys(this, core::mem::transmute_copy(&query)).into()
            }
        }
        unsafe extern "system" fn get_RegistryMaxRecursiveDepth<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::get_RegistryMaxRecursiveDepth(this) {
                    Ok(ok__) => {
                        depth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RegistryMaxRecursiveDepth<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::put_RegistryMaxRecursiveDepth(this, core::mem::transmute_copy(&depth)).into()
            }
        }
        unsafe extern "system" fn get_SystemStateFile<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::get_SystemStateFile(this) {
                    Ok(ok__) => {
                        filename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SystemStateFile<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::put_SystemStateFile(this, core::mem::transmute(&filename)).into()
            }
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            get_FileMaxCount: get_FileMaxCount::<Identity, OFFSET>,
            put_FileMaxCount: put_FileMaxCount::<Identity, OFFSET>,
            get_FileMaxRecursiveDepth: get_FileMaxRecursiveDepth::<Identity, OFFSET>,
            put_FileMaxRecursiveDepth: put_FileMaxRecursiveDepth::<Identity, OFFSET>,
            get_FileMaxTotalSize: get_FileMaxTotalSize::<Identity, OFFSET>,
            put_FileMaxTotalSize: put_FileMaxTotalSize::<Identity, OFFSET>,
            get_Files: get_Files::<Identity, OFFSET>,
            put_Files: put_Files::<Identity, OFFSET>,
            get_ManagementQueries: get_ManagementQueries::<Identity, OFFSET>,
            put_ManagementQueries: put_ManagementQueries::<Identity, OFFSET>,
            get_QueryNetworkAdapters: get_QueryNetworkAdapters::<Identity, OFFSET>,
            put_QueryNetworkAdapters: put_QueryNetworkAdapters::<Identity, OFFSET>,
            get_RegistryKeys: get_RegistryKeys::<Identity, OFFSET>,
            put_RegistryKeys: put_RegistryKeys::<Identity, OFFSET>,
            get_RegistryMaxRecursiveDepth: get_RegistryMaxRecursiveDepth::<Identity, OFFSET>,
            put_RegistryMaxRecursiveDepth: put_RegistryMaxRecursiveDepth::<Identity, OFFSET>,
            get_SystemStateFile: get_SystemStateFile::<Identity, OFFSET>,
            put_SystemStateFile: put_SystemStateFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConfigurationDataCollector as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IConfigurationDataCollector {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDataCollector, IDataCollector_Vtbl, 0x038374ff_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDataCollector {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDataCollector, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IDataCollector {
    pub unsafe fn get_DataCollectorSet(&self) -> windows_core::Result<IDataCollectorSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DataCollectorSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_DataCollectorSet<P0>(&self, group: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDataCollectorSet>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_DataCollectorSet)(windows_core::Interface::as_raw(self), group.param().abi()) }
    }
    pub unsafe fn get_DataCollectorType(&self) -> windows_core::Result<DataCollectorType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DataCollectorType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_FileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_FileName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn get_FileNameFormat(&self) -> windows_core::Result<AutoPathFormat> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FileNameFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_FileNameFormat(&self, format: AutoPathFormat) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FileNameFormat)(windows_core::Interface::as_raw(self), format) }
    }
    pub unsafe fn get_FileNameFormatPattern(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FileNameFormatPattern)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_FileNameFormatPattern(&self, pattern: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FileNameFormatPattern)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pattern)) }
    }
    pub unsafe fn get_LatestOutputLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LatestOutputLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_LatestOutputLocation(&self, path: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LatestOutputLocation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_LogAppend(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LogAppend)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_LogAppend(&self, append: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LogAppend)(windows_core::Interface::as_raw(self), append) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_LogCircular(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LogCircular)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_LogCircular(&self, circular: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LogCircular)(windows_core::Interface::as_raw(self), circular) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_LogOverwrite(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LogOverwrite)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_LogOverwrite(&self, overwrite: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LogOverwrite)(windows_core::Interface::as_raw(self), overwrite) }
    }
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Name)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn get_OutputLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OutputLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Index(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Index)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Index(&self, index: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Index)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn get_Xml(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Xml)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetXml(&self, xml: &windows_core::BSTR) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetXml)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(xml), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CreateOutputLocation(&self, latest: super::VARIANT_BOOL) -> windows_core::Result<windows_core::BSTR> {
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
    pub base__: super::IDispatch_Vtbl,
    pub get_DataCollectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_DataCollectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DataCollectorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataCollectorType) -> windows_core::HRESULT,
    pub get_FileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_FileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_FileNameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoPathFormat) -> windows_core::HRESULT,
    pub put_FileNameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, AutoPathFormat) -> windows_core::HRESULT,
    pub get_FileNameFormatPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_FileNameFormatPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_LatestOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_LatestOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_LogAppend: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_LogAppend: usize,
    #[cfg(feature = "wtypes")]
    pub put_LogAppend: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_LogAppend: usize,
    #[cfg(feature = "wtypes")]
    pub get_LogCircular: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_LogCircular: usize,
    #[cfg(feature = "wtypes")]
    pub put_LogCircular: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_LogCircular: usize,
    #[cfg(feature = "wtypes")]
    pub get_LogOverwrite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_LogOverwrite: usize,
    #[cfg(feature = "wtypes")]
    pub put_LogOverwrite: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_LogOverwrite: usize,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_OutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Index: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Xml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub CreateOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CreateOutputLocation: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDataCollector_Impl: super::IDispatch_Impl {
    fn get_DataCollectorSet(&self) -> windows_core::Result<IDataCollectorSet>;
    fn put_DataCollectorSet(&self, group: windows_core::Ref<IDataCollectorSet>) -> windows_core::Result<()>;
    fn get_DataCollectorType(&self) -> windows_core::Result<DataCollectorType>;
    fn get_FileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_FileName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_FileNameFormat(&self) -> windows_core::Result<AutoPathFormat>;
    fn put_FileNameFormat(&self, format: AutoPathFormat) -> windows_core::Result<()>;
    fn get_FileNameFormatPattern(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_FileNameFormatPattern(&self, pattern: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_LatestOutputLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_LatestOutputLocation(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_LogAppend(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_LogAppend(&self, append: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_LogCircular(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_LogCircular(&self, circular: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_LogOverwrite(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_LogOverwrite(&self, overwrite: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_OutputLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Index(&self) -> windows_core::Result<i32>;
    fn put_Index(&self, index: i32) -> windows_core::Result<()>;
    fn get_Xml(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetXml(&self, xml: &windows_core::BSTR) -> windows_core::Result<IValueMap>;
    fn CreateOutputLocation(&self, latest: super::VARIANT_BOOL) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDataCollector_Vtbl {
    pub const fn new<Identity: IDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DataCollectorSet<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::get_DataCollectorSet(this) {
                    Ok(ok__) => {
                        group.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DataCollectorSet<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::put_DataCollectorSet(this, core::mem::transmute_copy(&group)).into()
            }
        }
        unsafe extern "system" fn get_DataCollectorType<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut DataCollectorType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::get_DataCollectorType(this) {
                    Ok(ok__) => {
                        r#type.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FileName<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::get_FileName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FileName<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::put_FileName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn get_FileNameFormat<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut AutoPathFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::get_FileNameFormat(this) {
                    Ok(ok__) => {
                        format.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FileNameFormat<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: AutoPathFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::put_FileNameFormat(this, core::mem::transmute_copy(&format)).into()
            }
        }
        unsafe extern "system" fn get_FileNameFormatPattern<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::get_FileNameFormatPattern(this) {
                    Ok(ok__) => {
                        pattern.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FileNameFormatPattern<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::put_FileNameFormatPattern(this, core::mem::transmute(&pattern)).into()
            }
        }
        unsafe extern "system" fn get_LatestOutputLocation<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::get_LatestOutputLocation(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LatestOutputLocation<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::put_LatestOutputLocation(this, core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn get_LogAppend<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, append: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::get_LogAppend(this) {
                    Ok(ok__) => {
                        append.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LogAppend<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, append: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::put_LogAppend(this, core::mem::transmute_copy(&append)).into()
            }
        }
        unsafe extern "system" fn get_LogCircular<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, circular: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::get_LogCircular(this) {
                    Ok(ok__) => {
                        circular.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LogCircular<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, circular: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::put_LogCircular(this, core::mem::transmute_copy(&circular)).into()
            }
        }
        unsafe extern "system" fn get_LogOverwrite<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwrite: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::get_LogOverwrite(this) {
                    Ok(ok__) => {
                        overwrite.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LogOverwrite<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwrite: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::put_LogOverwrite(this, core::mem::transmute_copy(&overwrite)).into()
            }
        }
        unsafe extern "system" fn get_Name<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::get_Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Name<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::put_Name(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn get_OutputLocation<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::get_OutputLocation(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Index<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::get_Index(this) {
                    Ok(ok__) => {
                        index.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Index<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::put_Index(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn get_Xml<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::get_Xml(this) {
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
        unsafe extern "system" fn CreateOutputLocation<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, latest: super::VARIANT_BOOL, location: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_DataCollectorSet: get_DataCollectorSet::<Identity, OFFSET>,
            put_DataCollectorSet: put_DataCollectorSet::<Identity, OFFSET>,
            get_DataCollectorType: get_DataCollectorType::<Identity, OFFSET>,
            get_FileName: get_FileName::<Identity, OFFSET>,
            put_FileName: put_FileName::<Identity, OFFSET>,
            get_FileNameFormat: get_FileNameFormat::<Identity, OFFSET>,
            put_FileNameFormat: put_FileNameFormat::<Identity, OFFSET>,
            get_FileNameFormatPattern: get_FileNameFormatPattern::<Identity, OFFSET>,
            put_FileNameFormatPattern: put_FileNameFormatPattern::<Identity, OFFSET>,
            get_LatestOutputLocation: get_LatestOutputLocation::<Identity, OFFSET>,
            put_LatestOutputLocation: put_LatestOutputLocation::<Identity, OFFSET>,
            get_LogAppend: get_LogAppend::<Identity, OFFSET>,
            put_LogAppend: put_LogAppend::<Identity, OFFSET>,
            get_LogCircular: get_LogCircular::<Identity, OFFSET>,
            put_LogCircular: put_LogCircular::<Identity, OFFSET>,
            get_LogOverwrite: get_LogOverwrite::<Identity, OFFSET>,
            put_LogOverwrite: put_LogOverwrite::<Identity, OFFSET>,
            get_Name: get_Name::<Identity, OFFSET>,
            put_Name: put_Name::<Identity, OFFSET>,
            get_OutputLocation: get_OutputLocation::<Identity, OFFSET>,
            get_Index: get_Index::<Identity, OFFSET>,
            put_Index: put_Index::<Identity, OFFSET>,
            get_Xml: get_Xml::<Identity, OFFSET>,
            SetXml: SetXml::<Identity, OFFSET>,
            CreateOutputLocation: CreateOutputLocation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataCollector as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDataCollector {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDataCollectorCollection, IDataCollectorCollection_Vtbl, 0x03837502_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDataCollectorCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDataCollectorCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IDataCollectorCollection {
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<IDataCollector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, collector: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDataCollector>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), collector.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Remove(&self, collector: &super::VARIANT) -> windows_core::HRESULT {
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
            (windows_core::Interface::vtable(self).CreateDataCollectorFromXml)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxml), core::mem::transmute(pvalidation), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDataCollector(&self, r#type: DataCollectorType) -> windows_core::Result<IDataCollector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDataCollector)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDataCollectorFromXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDataCollector: unsafe extern "system" fn(*mut core::ffi::c_void, DataCollectorType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDataCollectorCollection_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<IDataCollector>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, collector: windows_core::Ref<IDataCollector>) -> windows_core::Result<()>;
    fn Remove(&self, collector: super::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, collectors: windows_core::Ref<IDataCollectorCollection>) -> windows_core::Result<()>;
    fn CreateDataCollectorFromXml(&self, bstrxml: &windows_core::BSTR, pvalidation: windows_core::OutRef<IValueMap>) -> windows_core::Result<IDataCollector>;
    fn CreateDataCollector(&self, r#type: DataCollectorType) -> windows_core::Result<IDataCollector>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDataCollectorCollection_Vtbl {
    pub const fn new<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::VARIANT, collector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        collector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorCollection_Impl::get__NewEnum(this) {
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
        unsafe extern "system" fn Remove<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collector: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorCollection_Impl::Remove(this, core::mem::transmute_copy(&collector)).into()
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Count: get_Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateDataCollectorFromXml: CreateDataCollectorFromXml::<Identity, OFFSET>,
            CreateDataCollector: CreateDataCollector::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataCollectorCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDataCollectorCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDataCollectorSet, IDataCollectorSet_Vtbl, 0x03837520_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDataCollectorSet {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDataCollectorSet, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IDataCollectorSet {
    pub unsafe fn get_DataCollectors(&self) -> windows_core::Result<IDataCollectorCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DataCollectors)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Duration(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Duration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Duration(&self, seconds: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Duration)(windows_core::Interface::as_raw(self), seconds) }
    }
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Description(&self, description: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description)) }
    }
    pub unsafe fn get_DescriptionUnresolved(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DescriptionUnresolved)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_DisplayName(&self, displayname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(displayname)) }
    }
    pub unsafe fn get_DisplayNameUnresolved(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DisplayNameUnresolved)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_Keywords(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Keywords)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_Keywords(&self, keywords: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Keywords)(windows_core::Interface::as_raw(self), keywords) }
    }
    pub unsafe fn get_LatestOutputLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LatestOutputLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_LatestOutputLocation(&self, path: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LatestOutputLocation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)) }
    }
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_OutputLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OutputLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_RootPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RootPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RootPath(&self, folder: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RootPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(folder)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Segment(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Segment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Segment(&self, segment: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Segment)(windows_core::Interface::as_raw(self), segment) }
    }
    pub unsafe fn get_SegmentMaxDuration(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SegmentMaxDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_SegmentMaxDuration(&self, seconds: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SegmentMaxDuration)(windows_core::Interface::as_raw(self), seconds) }
    }
    pub unsafe fn get_SegmentMaxSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SegmentMaxSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_SegmentMaxSize(&self, size: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SegmentMaxSize)(windows_core::Interface::as_raw(self), size) }
    }
    pub unsafe fn get_SerialNumber(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SerialNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_SerialNumber(&self, index: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SerialNumber)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn get_Server(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Server)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Status(&self) -> windows_core::Result<DataCollectorSetStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Subdirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Subdirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Subdirectory(&self, folder: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Subdirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(folder)) }
    }
    pub unsafe fn get_SubdirectoryFormat(&self) -> windows_core::Result<AutoPathFormat> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SubdirectoryFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_SubdirectoryFormat(&self, format: AutoPathFormat) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SubdirectoryFormat)(windows_core::Interface::as_raw(self), format) }
    }
    pub unsafe fn get_SubdirectoryFormatPattern(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SubdirectoryFormatPattern)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_SubdirectoryFormatPattern(&self, pattern: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SubdirectoryFormatPattern)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pattern)) }
    }
    pub unsafe fn get_Task(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Task)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Task(&self, task: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Task)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(task)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_TaskRunAsSelf(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TaskRunAsSelf)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_TaskRunAsSelf(&self, runasself: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TaskRunAsSelf)(windows_core::Interface::as_raw(self), runasself) }
    }
    pub unsafe fn get_TaskArguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TaskArguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_TaskArguments(&self, task: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TaskArguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(task)) }
    }
    pub unsafe fn get_TaskUserTextArguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TaskUserTextArguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_TaskUserTextArguments(&self, usertext: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TaskUserTextArguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(usertext)) }
    }
    pub unsafe fn get_Schedules(&self) -> windows_core::Result<IScheduleCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Schedules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_SchedulesEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SchedulesEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_SchedulesEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SchedulesEnabled)(windows_core::Interface::as_raw(self), enabled) }
    }
    pub unsafe fn get_UserAccount(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UserAccount)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Xml(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Xml)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Security(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Security)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Security(&self, bstrsecurity: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Security)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsecurity)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_StopOnCompletion(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StopOnCompletion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_StopOnCompletion(&self, stop: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StopOnCompletion)(windows_core::Interface::as_raw(self), stop) }
    }
    pub unsafe fn get_DataManager(&self) -> windows_core::Result<IDataManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DataManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(server), mode, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Start(&self, synchronous: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), synchronous) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Stop(&self, synchronous: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self), synchronous) }
    }
    pub unsafe fn SetXml(&self, xml: &windows_core::BSTR) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetXml)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(xml), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    pub base__: super::IDispatch_Vtbl,
    pub get_DataCollectors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_Duration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DescriptionUnresolved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DisplayNameUnresolved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_Keywords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_Keywords: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_Keywords: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_Keywords: usize,
    pub get_LatestOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_LatestOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_OutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_RootPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RootPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_Segment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Segment: usize,
    #[cfg(feature = "wtypes")]
    pub put_Segment: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Segment: usize,
    pub get_SegmentMaxDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_SegmentMaxDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_SegmentMaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_SegmentMaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_SerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_SerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_Server: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataCollectorSetStatus) -> windows_core::HRESULT,
    pub get_Subdirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Subdirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SubdirectoryFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoPathFormat) -> windows_core::HRESULT,
    pub put_SubdirectoryFormat: unsafe extern "system" fn(*mut core::ffi::c_void, AutoPathFormat) -> windows_core::HRESULT,
    pub get_SubdirectoryFormatPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_SubdirectoryFormatPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Task: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Task: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_TaskRunAsSelf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_TaskRunAsSelf: usize,
    #[cfg(feature = "wtypes")]
    pub put_TaskRunAsSelf: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_TaskRunAsSelf: usize,
    pub get_TaskArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_TaskArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_TaskUserTextArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_TaskUserTextArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Schedules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_SchedulesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_SchedulesEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_SchedulesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_SchedulesEnabled: usize,
    pub get_UserAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Xml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Security: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Security: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_StopOnCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_StopOnCompletion: usize,
    #[cfg(feature = "wtypes")]
    pub put_StopOnCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_StopOnCompletion: usize,
    pub get_DataManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, CommitMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Start: usize,
    #[cfg(feature = "wtypes")]
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Stop: usize,
    pub SetXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDataCollectorSet_Impl: super::IDispatch_Impl {
    fn get_DataCollectors(&self) -> windows_core::Result<IDataCollectorCollection>;
    fn get_Duration(&self) -> windows_core::Result<u32>;
    fn put_Duration(&self, seconds: u32) -> windows_core::Result<()>;
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Description(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_DescriptionUnresolved(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_DisplayName(&self, displayname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_DisplayNameUnresolved(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Keywords(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_Keywords(&self, keywords: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_LatestOutputLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_LatestOutputLocation(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_OutputLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_RootPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RootPath(&self, folder: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Segment(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Segment(&self, segment: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_SegmentMaxDuration(&self) -> windows_core::Result<u32>;
    fn put_SegmentMaxDuration(&self, seconds: u32) -> windows_core::Result<()>;
    fn get_SegmentMaxSize(&self) -> windows_core::Result<u32>;
    fn put_SegmentMaxSize(&self, size: u32) -> windows_core::Result<()>;
    fn get_SerialNumber(&self) -> windows_core::Result<u32>;
    fn put_SerialNumber(&self, index: u32) -> windows_core::Result<()>;
    fn get_Server(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Status(&self) -> windows_core::Result<DataCollectorSetStatus>;
    fn get_Subdirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Subdirectory(&self, folder: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_SubdirectoryFormat(&self) -> windows_core::Result<AutoPathFormat>;
    fn put_SubdirectoryFormat(&self, format: AutoPathFormat) -> windows_core::Result<()>;
    fn get_SubdirectoryFormatPattern(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_SubdirectoryFormatPattern(&self, pattern: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Task(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Task(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_TaskRunAsSelf(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_TaskRunAsSelf(&self, runasself: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_TaskArguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_TaskArguments(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_TaskUserTextArguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_TaskUserTextArguments(&self, usertext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Schedules(&self) -> windows_core::Result<IScheduleCollection>;
    fn get_SchedulesEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_SchedulesEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_UserAccount(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Xml(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Security(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Security(&self, bstrsecurity: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_StopOnCompletion(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_StopOnCompletion(&self, stop: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_DataManager(&self) -> windows_core::Result<IDataManager>;
    fn SetCredentials(&self, user: &windows_core::BSTR, password: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Query(&self, name: &windows_core::BSTR, server: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Commit(&self, name: &windows_core::BSTR, server: &windows_core::BSTR, mode: CommitMode) -> windows_core::Result<IValueMap>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Start(&self, synchronous: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Stop(&self, synchronous: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetXml(&self, xml: &windows_core::BSTR) -> windows_core::Result<IValueMap>;
    fn SetValue(&self, key: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetValue(&self, key: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDataCollectorSet_Vtbl {
    pub const fn new<Identity: IDataCollectorSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DataCollectors<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collectors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_DataCollectors(this) {
                    Ok(ok__) => {
                        collectors.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Duration<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_Duration(this) {
                    Ok(ok__) => {
                        seconds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Duration<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_Duration(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn get_Description<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Description<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_Description(this, core::mem::transmute(&description)).into()
            }
        }
        unsafe extern "system" fn get_DescriptionUnresolved<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, descr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_DescriptionUnresolved(this) {
                    Ok(ok__) => {
                        descr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DisplayName<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_DisplayName(this) {
                    Ok(ok__) => {
                        displayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DisplayName<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_DisplayName(this, core::mem::transmute(&displayname)).into()
            }
        }
        unsafe extern "system" fn get_DisplayNameUnresolved<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_DisplayNameUnresolved(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Keywords<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keywords: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_Keywords(this) {
                    Ok(ok__) => {
                        keywords.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Keywords<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keywords: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_Keywords(this, core::mem::transmute_copy(&keywords)).into()
            }
        }
        unsafe extern "system" fn get_LatestOutputLocation<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_LatestOutputLocation(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LatestOutputLocation<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_LatestOutputLocation(this, core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn get_Name<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_OutputLocation<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_OutputLocation(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RootPath<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_RootPath(this) {
                    Ok(ok__) => {
                        folder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RootPath<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_RootPath(this, core::mem::transmute(&folder)).into()
            }
        }
        unsafe extern "system" fn get_Segment<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segment: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_Segment(this) {
                    Ok(ok__) => {
                        segment.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Segment<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segment: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_Segment(this, core::mem::transmute_copy(&segment)).into()
            }
        }
        unsafe extern "system" fn get_SegmentMaxDuration<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_SegmentMaxDuration(this) {
                    Ok(ok__) => {
                        seconds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SegmentMaxDuration<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_SegmentMaxDuration(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn get_SegmentMaxSize<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_SegmentMaxSize(this) {
                    Ok(ok__) => {
                        size.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SegmentMaxSize<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_SegmentMaxSize(this, core::mem::transmute_copy(&size)).into()
            }
        }
        unsafe extern "system" fn get_SerialNumber<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_SerialNumber(this) {
                    Ok(ok__) => {
                        index.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SerialNumber<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_SerialNumber(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn get_Server<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_Server(this) {
                    Ok(ok__) => {
                        server.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Status<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut DataCollectorSetStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_Status(this) {
                    Ok(ok__) => {
                        status.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Subdirectory<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_Subdirectory(this) {
                    Ok(ok__) => {
                        folder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Subdirectory<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_Subdirectory(this, core::mem::transmute(&folder)).into()
            }
        }
        unsafe extern "system" fn get_SubdirectoryFormat<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut AutoPathFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_SubdirectoryFormat(this) {
                    Ok(ok__) => {
                        format.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SubdirectoryFormat<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: AutoPathFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_SubdirectoryFormat(this, core::mem::transmute_copy(&format)).into()
            }
        }
        unsafe extern "system" fn get_SubdirectoryFormatPattern<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_SubdirectoryFormatPattern(this) {
                    Ok(ok__) => {
                        pattern.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SubdirectoryFormatPattern<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_SubdirectoryFormatPattern(this, core::mem::transmute(&pattern)).into()
            }
        }
        unsafe extern "system" fn get_Task<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_Task(this) {
                    Ok(ok__) => {
                        task.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Task<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_Task(this, core::mem::transmute(&task)).into()
            }
        }
        unsafe extern "system" fn get_TaskRunAsSelf<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runasself: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_TaskRunAsSelf(this) {
                    Ok(ok__) => {
                        runasself.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TaskRunAsSelf<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runasself: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_TaskRunAsSelf(this, core::mem::transmute_copy(&runasself)).into()
            }
        }
        unsafe extern "system" fn get_TaskArguments<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_TaskArguments(this) {
                    Ok(ok__) => {
                        task.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TaskArguments<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_TaskArguments(this, core::mem::transmute(&task)).into()
            }
        }
        unsafe extern "system" fn get_TaskUserTextArguments<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usertext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_TaskUserTextArguments(this) {
                    Ok(ok__) => {
                        usertext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TaskUserTextArguments<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usertext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_TaskUserTextArguments(this, core::mem::transmute(&usertext)).into()
            }
        }
        unsafe extern "system" fn get_Schedules<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppschedules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_Schedules(this) {
                    Ok(ok__) => {
                        ppschedules.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SchedulesEnabled<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_SchedulesEnabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SchedulesEnabled<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_SchedulesEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn get_UserAccount<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, user: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_UserAccount(this) {
                    Ok(ok__) => {
                        user.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Xml<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_Xml(this) {
                    Ok(ok__) => {
                        xml.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Security<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_Security(this) {
                    Ok(ok__) => {
                        pbstrsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Security<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsecurity: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_Security(this, core::mem::transmute(&bstrsecurity)).into()
            }
        }
        unsafe extern "system" fn get_StopOnCompletion<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_StopOnCompletion(this) {
                    Ok(ok__) => {
                        stop.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StopOnCompletion<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::put_StopOnCompletion(this, core::mem::transmute_copy(&stop)).into()
            }
        }
        unsafe extern "system" fn get_DataManager<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datamanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::get_DataManager(this) {
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
        unsafe extern "system" fn Start<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, synchronous: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::Start(this, core::mem::transmute_copy(&synchronous)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, synchronous: super::VARIANT_BOOL) -> windows_core::HRESULT {
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_DataCollectors: get_DataCollectors::<Identity, OFFSET>,
            get_Duration: get_Duration::<Identity, OFFSET>,
            put_Duration: put_Duration::<Identity, OFFSET>,
            get_Description: get_Description::<Identity, OFFSET>,
            put_Description: put_Description::<Identity, OFFSET>,
            get_DescriptionUnresolved: get_DescriptionUnresolved::<Identity, OFFSET>,
            get_DisplayName: get_DisplayName::<Identity, OFFSET>,
            put_DisplayName: put_DisplayName::<Identity, OFFSET>,
            get_DisplayNameUnresolved: get_DisplayNameUnresolved::<Identity, OFFSET>,
            get_Keywords: get_Keywords::<Identity, OFFSET>,
            put_Keywords: put_Keywords::<Identity, OFFSET>,
            get_LatestOutputLocation: get_LatestOutputLocation::<Identity, OFFSET>,
            put_LatestOutputLocation: put_LatestOutputLocation::<Identity, OFFSET>,
            get_Name: get_Name::<Identity, OFFSET>,
            get_OutputLocation: get_OutputLocation::<Identity, OFFSET>,
            get_RootPath: get_RootPath::<Identity, OFFSET>,
            put_RootPath: put_RootPath::<Identity, OFFSET>,
            get_Segment: get_Segment::<Identity, OFFSET>,
            put_Segment: put_Segment::<Identity, OFFSET>,
            get_SegmentMaxDuration: get_SegmentMaxDuration::<Identity, OFFSET>,
            put_SegmentMaxDuration: put_SegmentMaxDuration::<Identity, OFFSET>,
            get_SegmentMaxSize: get_SegmentMaxSize::<Identity, OFFSET>,
            put_SegmentMaxSize: put_SegmentMaxSize::<Identity, OFFSET>,
            get_SerialNumber: get_SerialNumber::<Identity, OFFSET>,
            put_SerialNumber: put_SerialNumber::<Identity, OFFSET>,
            get_Server: get_Server::<Identity, OFFSET>,
            get_Status: get_Status::<Identity, OFFSET>,
            get_Subdirectory: get_Subdirectory::<Identity, OFFSET>,
            put_Subdirectory: put_Subdirectory::<Identity, OFFSET>,
            get_SubdirectoryFormat: get_SubdirectoryFormat::<Identity, OFFSET>,
            put_SubdirectoryFormat: put_SubdirectoryFormat::<Identity, OFFSET>,
            get_SubdirectoryFormatPattern: get_SubdirectoryFormatPattern::<Identity, OFFSET>,
            put_SubdirectoryFormatPattern: put_SubdirectoryFormatPattern::<Identity, OFFSET>,
            get_Task: get_Task::<Identity, OFFSET>,
            put_Task: put_Task::<Identity, OFFSET>,
            get_TaskRunAsSelf: get_TaskRunAsSelf::<Identity, OFFSET>,
            put_TaskRunAsSelf: put_TaskRunAsSelf::<Identity, OFFSET>,
            get_TaskArguments: get_TaskArguments::<Identity, OFFSET>,
            put_TaskArguments: put_TaskArguments::<Identity, OFFSET>,
            get_TaskUserTextArguments: get_TaskUserTextArguments::<Identity, OFFSET>,
            put_TaskUserTextArguments: put_TaskUserTextArguments::<Identity, OFFSET>,
            get_Schedules: get_Schedules::<Identity, OFFSET>,
            get_SchedulesEnabled: get_SchedulesEnabled::<Identity, OFFSET>,
            put_SchedulesEnabled: put_SchedulesEnabled::<Identity, OFFSET>,
            get_UserAccount: get_UserAccount::<Identity, OFFSET>,
            get_Xml: get_Xml::<Identity, OFFSET>,
            get_Security: get_Security::<Identity, OFFSET>,
            put_Security: put_Security::<Identity, OFFSET>,
            get_StopOnCompletion: get_StopOnCompletion::<Identity, OFFSET>,
            put_StopOnCompletion: put_StopOnCompletion::<Identity, OFFSET>,
            get_DataManager: get_DataManager::<Identity, OFFSET>,
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
        iid == &<IDataCollectorSet as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDataCollectorSet {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDataCollectorSetCollection, IDataCollectorSetCollection_Vtbl, 0x03837524_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDataCollectorSetCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDataCollectorSetCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IDataCollectorSetCollection {
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<IDataCollectorSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, set: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDataCollectorSet>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), set.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Remove(&self, set: &super::VARIANT) -> windows_core::HRESULT {
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
    pub base__: super::IDispatch_Vtbl,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDataCollectorSets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDataCollectorSetCollection_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<IDataCollectorSet>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, set: windows_core::Ref<IDataCollectorSet>) -> windows_core::Result<()>;
    fn Remove(&self, set: super::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, sets: windows_core::Ref<IDataCollectorSetCollection>) -> windows_core::Result<()>;
    fn GetDataCollectorSets(&self, server: &windows_core::BSTR, filter: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDataCollectorSetCollection_Vtbl {
    pub const fn new<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSetCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::VARIANT, set: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSetCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        set.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSetCollection_Impl::get__NewEnum(this) {
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
        unsafe extern "system" fn Remove<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, set: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSetCollection_Impl::Remove(this, core::mem::transmute_copy(&set)).into()
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Count: get_Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            GetDataCollectorSets: GetDataCollectorSets::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataCollectorSetCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDataCollectorSetCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDataManager, IDataManager_Vtbl, 0x03837541_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDataManager {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDataManager, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IDataManager {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Enabled(&self, fenabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Enabled)(windows_core::Interface::as_raw(self), fenabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_CheckBeforeRunning(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CheckBeforeRunning)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_CheckBeforeRunning(&self, fcheck: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_CheckBeforeRunning)(windows_core::Interface::as_raw(self), fcheck) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_MinFreeDisk(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MinFreeDisk)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_MinFreeDisk(&self, minfreedisk: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MinFreeDisk)(windows_core::Interface::as_raw(self), minfreedisk) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_MaxSize(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MaxSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_MaxSize(&self, ulmaxsize: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MaxSize)(windows_core::Interface::as_raw(self), ulmaxsize) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_MaxFolderCount(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MaxFolderCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_MaxFolderCount(&self, ulmaxfoldercount: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MaxFolderCount)(windows_core::Interface::as_raw(self), ulmaxfoldercount) }
    }
    pub unsafe fn get_ResourcePolicy(&self) -> windows_core::Result<ResourcePolicy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ResourcePolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ResourcePolicy(&self, policy: ResourcePolicy) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ResourcePolicy)(windows_core::Interface::as_raw(self), policy) }
    }
    pub unsafe fn get_FolderActions(&self) -> windows_core::Result<IFolderActionCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FolderActions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ReportSchema(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReportSchema)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ReportSchema(&self, reportschema: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ReportSchema)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(reportschema)) }
    }
    pub unsafe fn get_ReportFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReportFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ReportFileName(&self, pbstrfilename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ReportFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pbstrfilename)) }
    }
    pub unsafe fn get_RuleTargetFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RuleTargetFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RuleTargetFileName(&self, filename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RuleTargetFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filename)) }
    }
    pub unsafe fn get_EventsFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EventsFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_EventsFileName(&self, pbstrfilename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_EventsFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pbstrfilename)) }
    }
    pub unsafe fn get_Rules(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Rules)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Rules(&self, bstrxml: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Rules)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxml)) }
    }
    pub unsafe fn Run(&self, steps: DataManagerSteps, bstrfolder: &windows_core::BSTR) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self), steps, core::mem::transmute_copy(bstrfolder), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub get_CheckBeforeRunning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_CheckBeforeRunning: usize,
    #[cfg(feature = "wtypes")]
    pub put_CheckBeforeRunning: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_CheckBeforeRunning: usize,
    #[cfg(feature = "shared")]
    pub get_MinFreeDisk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_MinFreeDisk: usize,
    #[cfg(feature = "shared")]
    pub put_MinFreeDisk: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_MinFreeDisk: usize,
    #[cfg(feature = "shared")]
    pub get_MaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_MaxSize: usize,
    #[cfg(feature = "shared")]
    pub put_MaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_MaxSize: usize,
    #[cfg(feature = "shared")]
    pub get_MaxFolderCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_MaxFolderCount: usize,
    #[cfg(feature = "shared")]
    pub put_MaxFolderCount: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_MaxFolderCount: usize,
    pub get_ResourcePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ResourcePolicy) -> windows_core::HRESULT,
    pub put_ResourcePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, ResourcePolicy) -> windows_core::HRESULT,
    pub get_FolderActions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ReportSchema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ReportSchema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ReportFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ReportFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_RuleTargetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RuleTargetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_EventsFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_EventsFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Rules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Rules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, DataManagerSteps, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Extract: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDataManager_Impl: super::IDispatch_Impl {
    fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Enabled(&self, fenabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_CheckBeforeRunning(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_CheckBeforeRunning(&self, fcheck: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_MinFreeDisk(&self) -> windows_core::Result<super::ULONG>;
    fn put_MinFreeDisk(&self, minfreedisk: super::ULONG) -> windows_core::Result<()>;
    fn get_MaxSize(&self) -> windows_core::Result<super::ULONG>;
    fn put_MaxSize(&self, ulmaxsize: super::ULONG) -> windows_core::Result<()>;
    fn get_MaxFolderCount(&self) -> windows_core::Result<super::ULONG>;
    fn put_MaxFolderCount(&self, ulmaxfoldercount: super::ULONG) -> windows_core::Result<()>;
    fn get_ResourcePolicy(&self) -> windows_core::Result<ResourcePolicy>;
    fn put_ResourcePolicy(&self, policy: ResourcePolicy) -> windows_core::Result<()>;
    fn get_FolderActions(&self) -> windows_core::Result<IFolderActionCollection>;
    fn get_ReportSchema(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ReportSchema(&self, reportschema: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ReportFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ReportFileName(&self, pbstrfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_RuleTargetFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RuleTargetFileName(&self, filename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_EventsFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_EventsFileName(&self, pbstrfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Rules(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Rules(&self, bstrxml: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Run(&self, steps: DataManagerSteps, bstrfolder: &windows_core::BSTR) -> windows_core::Result<IValueMap>;
    fn Extract(&self, cabfilename: &windows_core::BSTR, destinationpath: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDataManager_Vtbl {
    pub const fn new<Identity: IDataManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Enabled<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::get_Enabled(this) {
                    Ok(ok__) => {
                        pfenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Enabled<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::put_Enabled(this, core::mem::transmute_copy(&fenabled)).into()
            }
        }
        unsafe extern "system" fn get_CheckBeforeRunning<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcheck: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::get_CheckBeforeRunning(this) {
                    Ok(ok__) => {
                        pfcheck.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_CheckBeforeRunning<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcheck: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::put_CheckBeforeRunning(this, core::mem::transmute_copy(&fcheck)).into()
            }
        }
        unsafe extern "system" fn get_MinFreeDisk<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minfreedisk: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::get_MinFreeDisk(this) {
                    Ok(ok__) => {
                        minfreedisk.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MinFreeDisk<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minfreedisk: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::put_MinFreeDisk(this, core::mem::transmute_copy(&minfreedisk)).into()
            }
        }
        unsafe extern "system" fn get_MaxSize<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulmaxsize: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::get_MaxSize(this) {
                    Ok(ok__) => {
                        pulmaxsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MaxSize<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmaxsize: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::put_MaxSize(this, core::mem::transmute_copy(&ulmaxsize)).into()
            }
        }
        unsafe extern "system" fn get_MaxFolderCount<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulmaxfoldercount: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::get_MaxFolderCount(this) {
                    Ok(ok__) => {
                        pulmaxfoldercount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MaxFolderCount<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmaxfoldercount: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::put_MaxFolderCount(this, core::mem::transmute_copy(&ulmaxfoldercount)).into()
            }
        }
        unsafe extern "system" fn get_ResourcePolicy<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppolicy: *mut ResourcePolicy) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::get_ResourcePolicy(this) {
                    Ok(ok__) => {
                        ppolicy.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ResourcePolicy<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: ResourcePolicy) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::put_ResourcePolicy(this, core::mem::transmute_copy(&policy)).into()
            }
        }
        unsafe extern "system" fn get_FolderActions<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::get_FolderActions(this) {
                    Ok(ok__) => {
                        actions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ReportSchema<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportschema: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::get_ReportSchema(this) {
                    Ok(ok__) => {
                        reportschema.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ReportSchema<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportschema: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::put_ReportSchema(this, core::mem::transmute(&reportschema)).into()
            }
        }
        unsafe extern "system" fn get_ReportFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::get_ReportFileName(this) {
                    Ok(ok__) => {
                        pbstrfilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ReportFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::put_ReportFileName(this, core::mem::transmute(&pbstrfilename)).into()
            }
        }
        unsafe extern "system" fn get_RuleTargetFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::get_RuleTargetFileName(this) {
                    Ok(ok__) => {
                        filename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RuleTargetFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::put_RuleTargetFileName(this, core::mem::transmute(&filename)).into()
            }
        }
        unsafe extern "system" fn get_EventsFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::get_EventsFileName(this) {
                    Ok(ok__) => {
                        pbstrfilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_EventsFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::put_EventsFileName(this, core::mem::transmute(&pbstrfilename)).into()
            }
        }
        unsafe extern "system" fn get_Rules<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::get_Rules(this) {
                    Ok(ok__) => {
                        pbstrxml.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Rules<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::put_Rules(this, core::mem::transmute(&bstrxml)).into()
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Enabled: get_Enabled::<Identity, OFFSET>,
            put_Enabled: put_Enabled::<Identity, OFFSET>,
            get_CheckBeforeRunning: get_CheckBeforeRunning::<Identity, OFFSET>,
            put_CheckBeforeRunning: put_CheckBeforeRunning::<Identity, OFFSET>,
            get_MinFreeDisk: get_MinFreeDisk::<Identity, OFFSET>,
            put_MinFreeDisk: put_MinFreeDisk::<Identity, OFFSET>,
            get_MaxSize: get_MaxSize::<Identity, OFFSET>,
            put_MaxSize: put_MaxSize::<Identity, OFFSET>,
            get_MaxFolderCount: get_MaxFolderCount::<Identity, OFFSET>,
            put_MaxFolderCount: put_MaxFolderCount::<Identity, OFFSET>,
            get_ResourcePolicy: get_ResourcePolicy::<Identity, OFFSET>,
            put_ResourcePolicy: put_ResourcePolicy::<Identity, OFFSET>,
            get_FolderActions: get_FolderActions::<Identity, OFFSET>,
            get_ReportSchema: get_ReportSchema::<Identity, OFFSET>,
            put_ReportSchema: put_ReportSchema::<Identity, OFFSET>,
            get_ReportFileName: get_ReportFileName::<Identity, OFFSET>,
            put_ReportFileName: put_ReportFileName::<Identity, OFFSET>,
            get_RuleTargetFileName: get_RuleTargetFileName::<Identity, OFFSET>,
            put_RuleTargetFileName: put_RuleTargetFileName::<Identity, OFFSET>,
            get_EventsFileName: get_EventsFileName::<Identity, OFFSET>,
            put_EventsFileName: put_EventsFileName::<Identity, OFFSET>,
            get_Rules: get_Rules::<Identity, OFFSET>,
            put_Rules: put_Rules::<Identity, OFFSET>,
            Run: Run::<Identity, OFFSET>,
            Extract: Extract::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataManager as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDataManager {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFolderAction, IFolderAction_Vtbl, 0x03837543_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFolderAction {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFolderAction, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFolderAction {
    #[cfg(feature = "shared")]
    pub unsafe fn get_Age(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Age)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_Age(&self, ulage: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Age)(windows_core::Interface::as_raw(self), ulage) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Size(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_Size(&self, ulage: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Size)(windows_core::Interface::as_raw(self), ulage) }
    }
    pub unsafe fn get_Actions(&self) -> windows_core::Result<FolderActionSteps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Actions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Actions(&self, steps: FolderActionSteps) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Actions)(windows_core::Interface::as_raw(self), steps) }
    }
    pub unsafe fn get_SendCabTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SendCabTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_SendCabTo(&self, bstrdestination: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SendCabTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdestination)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFolderAction_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_Age: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Age: usize,
    #[cfg(feature = "shared")]
    pub put_Age: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_Age: usize,
    #[cfg(feature = "shared")]
    pub get_Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Size: usize,
    #[cfg(feature = "shared")]
    pub put_Size: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_Size: usize,
    pub get_Actions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FolderActionSteps) -> windows_core::HRESULT,
    pub put_Actions: unsafe extern "system" fn(*mut core::ffi::c_void, FolderActionSteps) -> windows_core::HRESULT,
    pub get_SendCabTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_SendCabTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFolderAction_Impl: super::IDispatch_Impl {
    fn get_Age(&self) -> windows_core::Result<super::ULONG>;
    fn put_Age(&self, ulage: super::ULONG) -> windows_core::Result<()>;
    fn get_Size(&self) -> windows_core::Result<super::ULONG>;
    fn put_Size(&self, ulage: super::ULONG) -> windows_core::Result<()>;
    fn get_Actions(&self) -> windows_core::Result<FolderActionSteps>;
    fn put_Actions(&self, steps: FolderActionSteps) -> windows_core::Result<()>;
    fn get_SendCabTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_SendCabTo(&self, bstrdestination: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFolderAction_Vtbl {
    pub const fn new<Identity: IFolderAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Age<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulage: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderAction_Impl::get_Age(this) {
                    Ok(ok__) => {
                        pulage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Age<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulage: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderAction_Impl::put_Age(this, core::mem::transmute_copy(&ulage)).into()
            }
        }
        unsafe extern "system" fn get_Size<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulage: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderAction_Impl::get_Size(this) {
                    Ok(ok__) => {
                        pulage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Size<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulage: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderAction_Impl::put_Size(this, core::mem::transmute_copy(&ulage)).into()
            }
        }
        unsafe extern "system" fn get_Actions<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, steps: *mut FolderActionSteps) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderAction_Impl::get_Actions(this) {
                    Ok(ok__) => {
                        steps.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Actions<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, steps: FolderActionSteps) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderAction_Impl::put_Actions(this, core::mem::transmute_copy(&steps)).into()
            }
        }
        unsafe extern "system" fn get_SendCabTo<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdestination: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderAction_Impl::get_SendCabTo(this) {
                    Ok(ok__) => {
                        pbstrdestination.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SendCabTo<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdestination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderAction_Impl::put_SendCabTo(this, core::mem::transmute(&bstrdestination)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Age: get_Age::<Identity, OFFSET>,
            put_Age: put_Age::<Identity, OFFSET>,
            get_Size: get_Size::<Identity, OFFSET>,
            put_Size: put_Size::<Identity, OFFSET>,
            get_Actions: get_Actions::<Identity, OFFSET>,
            put_Actions: put_Actions::<Identity, OFFSET>,
            get_SendCabTo: get_SendCabTo::<Identity, OFFSET>,
            put_SendCabTo: put_SendCabTo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFolderAction as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFolderAction {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFolderActionCollection, IFolderActionCollection_Vtbl, 0x03837544_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFolderActionCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFolderActionCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFolderActionCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<IFolderAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, action: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IFolderAction>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), action.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Remove(&self, index: &super::VARIANT) -> windows_core::HRESULT {
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
            (windows_core::Interface::vtable(self).CreateFolderAction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFolderActionCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFolderAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFolderActionCollection_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<super::ULONG>;
    fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<IFolderAction>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, action: windows_core::Ref<IFolderAction>) -> windows_core::Result<()>;
    fn Remove(&self, index: super::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, actions: windows_core::Ref<IFolderActionCollection>) -> windows_core::Result<()>;
    fn CreateFolderAction(&self) -> windows_core::Result<IFolderAction>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFolderActionCollection_Vtbl {
    pub const fn new<Identity: IFolderActionCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderActionCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::VARIANT, action: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderActionCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        action.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#enum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderActionCollection_Impl::get__NewEnum(this) {
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
        unsafe extern "system" fn Remove<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderActionCollection_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Count: get_Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateFolderAction: CreateFolderAction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFolderActionCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IPerformanceCounterDataCollector, windows_core::IUnknown, super::IDispatch, IDataCollector);
#[cfg(feature = "oaidl")]
impl IPerformanceCounterDataCollector {
    pub unsafe fn get_DataSourceName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DataSourceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_DataSourceName(&self, dsn: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DataSourceName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(dsn)) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_PerformanceCounters(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PerformanceCounters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_PerformanceCounters(&self, counters: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PerformanceCounters)(windows_core::Interface::as_raw(self), counters) }
    }
    pub unsafe fn get_LogFileFormat(&self) -> windows_core::Result<FileFormat> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LogFileFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_LogFileFormat(&self, format: FileFormat) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LogFileFormat)(windows_core::Interface::as_raw(self), format) }
    }
    pub unsafe fn get_SampleInterval(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SampleInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_SampleInterval(&self, interval: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SampleInterval)(windows_core::Interface::as_raw(self), interval) }
    }
    pub unsafe fn get_SegmentMaxRecords(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SegmentMaxRecords)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_SegmentMaxRecords(&self, records: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SegmentMaxRecords)(windows_core::Interface::as_raw(self), records) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerformanceCounterDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    pub get_DataSourceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_DataSourceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_PerformanceCounters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_PerformanceCounters: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_PerformanceCounters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_PerformanceCounters: usize,
    pub get_LogFileFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FileFormat) -> windows_core::HRESULT,
    pub put_LogFileFormat: unsafe extern "system" fn(*mut core::ffi::c_void, FileFormat) -> windows_core::HRESULT,
    pub get_SampleInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_SampleInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_SegmentMaxRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_SegmentMaxRecords: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPerformanceCounterDataCollector_Impl: IDataCollector_Impl {
    fn get_DataSourceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_DataSourceName(&self, dsn: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_PerformanceCounters(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_PerformanceCounters(&self, counters: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_LogFileFormat(&self) -> windows_core::Result<FileFormat>;
    fn put_LogFileFormat(&self, format: FileFormat) -> windows_core::Result<()>;
    fn get_SampleInterval(&self) -> windows_core::Result<u32>;
    fn put_SampleInterval(&self, interval: u32) -> windows_core::Result<()>;
    fn get_SegmentMaxRecords(&self) -> windows_core::Result<u32>;
    fn put_SegmentMaxRecords(&self, records: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPerformanceCounterDataCollector_Vtbl {
    pub const fn new<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DataSourceName<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dsn: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerformanceCounterDataCollector_Impl::get_DataSourceName(this) {
                    Ok(ok__) => {
                        dsn.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DataSourceName<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dsn: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerformanceCounterDataCollector_Impl::put_DataSourceName(this, core::mem::transmute(&dsn)).into()
            }
        }
        unsafe extern "system" fn get_PerformanceCounters<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, counters: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerformanceCounterDataCollector_Impl::get_PerformanceCounters(this) {
                    Ok(ok__) => {
                        counters.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PerformanceCounters<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, counters: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerformanceCounterDataCollector_Impl::put_PerformanceCounters(this, core::mem::transmute_copy(&counters)).into()
            }
        }
        unsafe extern "system" fn get_LogFileFormat<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut FileFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerformanceCounterDataCollector_Impl::get_LogFileFormat(this) {
                    Ok(ok__) => {
                        format.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LogFileFormat<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: FileFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerformanceCounterDataCollector_Impl::put_LogFileFormat(this, core::mem::transmute_copy(&format)).into()
            }
        }
        unsafe extern "system" fn get_SampleInterval<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerformanceCounterDataCollector_Impl::get_SampleInterval(this) {
                    Ok(ok__) => {
                        interval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SampleInterval<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerformanceCounterDataCollector_Impl::put_SampleInterval(this, core::mem::transmute_copy(&interval)).into()
            }
        }
        unsafe extern "system" fn get_SegmentMaxRecords<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, records: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerformanceCounterDataCollector_Impl::get_SegmentMaxRecords(this) {
                    Ok(ok__) => {
                        records.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SegmentMaxRecords<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, records: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerformanceCounterDataCollector_Impl::put_SegmentMaxRecords(this, core::mem::transmute_copy(&records)).into()
            }
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            get_DataSourceName: get_DataSourceName::<Identity, OFFSET>,
            put_DataSourceName: put_DataSourceName::<Identity, OFFSET>,
            get_PerformanceCounters: get_PerformanceCounters::<Identity, OFFSET>,
            put_PerformanceCounters: put_PerformanceCounters::<Identity, OFFSET>,
            get_LogFileFormat: get_LogFileFormat::<Identity, OFFSET>,
            put_LogFileFormat: put_LogFileFormat::<Identity, OFFSET>,
            get_SampleInterval: get_SampleInterval::<Identity, OFFSET>,
            put_SampleInterval: put_SampleInterval::<Identity, OFFSET>,
            get_SegmentMaxRecords: get_SegmentMaxRecords::<Identity, OFFSET>,
            put_SegmentMaxRecords: put_SegmentMaxRecords::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPerformanceCounterDataCollector as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPerformanceCounterDataCollector {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISchedule, ISchedule_Vtbl, 0x0383753a_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISchedule {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISchedule, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISchedule {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_StartDate(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_StartDate(&self, start: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StartDate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(start)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_EndDate(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EndDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_EndDate(&self, end: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_EndDate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(end)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_StartTime(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_StartTime(&self, start: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StartTime)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(start)) }
    }
    pub unsafe fn get_Days(&self) -> windows_core::Result<WeekDays> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Days)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Days(&self, days: WeekDays) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Days)(windows_core::Interface::as_raw(self), days) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchedule_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_StartDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_StartDate: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_StartDate: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_StartDate: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_EndDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_EndDate: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_EndDate: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_EndDate: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_StartTime: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_StartTime: usize,
    pub get_Days: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WeekDays) -> windows_core::HRESULT,
    pub put_Days: unsafe extern "system" fn(*mut core::ffi::c_void, WeekDays) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISchedule_Impl: super::IDispatch_Impl {
    fn get_StartDate(&self) -> windows_core::Result<super::VARIANT>;
    fn put_StartDate(&self, start: &super::VARIANT) -> windows_core::Result<()>;
    fn get_EndDate(&self) -> windows_core::Result<super::VARIANT>;
    fn put_EndDate(&self, end: &super::VARIANT) -> windows_core::Result<()>;
    fn get_StartTime(&self) -> windows_core::Result<super::VARIANT>;
    fn put_StartTime(&self, start: &super::VARIANT) -> windows_core::Result<()>;
    fn get_Days(&self) -> windows_core::Result<WeekDays>;
    fn put_Days(&self, days: WeekDays) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISchedule_Vtbl {
    pub const fn new<Identity: ISchedule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_StartDate<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchedule_Impl::get_StartDate(this) {
                    Ok(ok__) => {
                        start.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StartDate<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchedule_Impl::put_StartDate(this, core::mem::transmute(&start)).into()
            }
        }
        unsafe extern "system" fn get_EndDate<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, end: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchedule_Impl::get_EndDate(this) {
                    Ok(ok__) => {
                        end.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_EndDate<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, end: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchedule_Impl::put_EndDate(this, core::mem::transmute(&end)).into()
            }
        }
        unsafe extern "system" fn get_StartTime<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchedule_Impl::get_StartTime(this) {
                    Ok(ok__) => {
                        start.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StartTime<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchedule_Impl::put_StartTime(this, core::mem::transmute(&start)).into()
            }
        }
        unsafe extern "system" fn get_Days<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: *mut WeekDays) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchedule_Impl::get_Days(this) {
                    Ok(ok__) => {
                        days.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Days<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: WeekDays) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchedule_Impl::put_Days(this, core::mem::transmute_copy(&days)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_StartDate: get_StartDate::<Identity, OFFSET>,
            put_StartDate: put_StartDate::<Identity, OFFSET>,
            get_EndDate: get_EndDate::<Identity, OFFSET>,
            put_EndDate: put_EndDate::<Identity, OFFSET>,
            get_StartTime: get_StartTime::<Identity, OFFSET>,
            put_StartTime: put_StartTime::<Identity, OFFSET>,
            get_Days: get_Days::<Identity, OFFSET>,
            put_Days: put_Days::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchedule as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISchedule {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IScheduleCollection, IScheduleCollection_Vtbl, 0x0383753d_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IScheduleCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IScheduleCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IScheduleCollection {
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<ISchedule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, pschedule: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISchedule>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pschedule.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Remove(&self, vschedule: &super::VARIANT) -> windows_core::HRESULT {
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
            (windows_core::Interface::vtable(self).CreateSchedule)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IScheduleCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSchedule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IScheduleCollection_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<ISchedule>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pschedule: windows_core::Ref<ISchedule>) -> windows_core::Result<()>;
    fn Remove(&self, vschedule: super::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, pschedules: windows_core::Ref<IScheduleCollection>) -> windows_core::Result<()>;
    fn CreateSchedule(&self) -> windows_core::Result<ISchedule>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IScheduleCollection_Vtbl {
    pub const fn new<Identity: IScheduleCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduleCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::VARIANT, ppschedule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduleCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        ppschedule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduleCollection_Impl::get__NewEnum(this) {
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
        unsafe extern "system" fn Remove<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vschedule: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduleCollection_Impl::Remove(this, core::mem::transmute_copy(&vschedule)).into()
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Count: get_Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateSchedule: CreateSchedule::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScheduleCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(ITraceDataCollector, windows_core::IUnknown, super::IDispatch, IDataCollector);
#[cfg(feature = "oaidl")]
impl ITraceDataCollector {
    pub unsafe fn get_BufferSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BufferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_BufferSize(&self, size: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BufferSize)(windows_core::Interface::as_raw(self), size) }
    }
    pub unsafe fn get_BuffersLost(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BuffersLost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_BuffersLost(&self, buffers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BuffersLost)(windows_core::Interface::as_raw(self), buffers) }
    }
    pub unsafe fn get_BuffersWritten(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BuffersWritten)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_BuffersWritten(&self, buffers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BuffersWritten)(windows_core::Interface::as_raw(self), buffers) }
    }
    pub unsafe fn get_ClockType(&self) -> windows_core::Result<ClockType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClockType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ClockType(&self, clock: ClockType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ClockType)(windows_core::Interface::as_raw(self), clock) }
    }
    pub unsafe fn get_EventsLost(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EventsLost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_EventsLost(&self, events: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_EventsLost)(windows_core::Interface::as_raw(self), events) }
    }
    pub unsafe fn get_ExtendedModes(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExtendedModes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ExtendedModes(&self, mode: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ExtendedModes)(windows_core::Interface::as_raw(self), mode) }
    }
    pub unsafe fn get_FlushTimer(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FlushTimer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_FlushTimer(&self, seconds: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FlushTimer)(windows_core::Interface::as_raw(self), seconds) }
    }
    pub unsafe fn get_FreeBuffers(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FreeBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_FreeBuffers(&self, buffers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FreeBuffers)(windows_core::Interface::as_raw(self), buffers) }
    }
    pub unsafe fn get_Guid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Guid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Guid(&self, guid: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Guid)(windows_core::Interface::as_raw(self), guid) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsKernelTrace(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsKernelTrace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_MaximumBuffers(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MaximumBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MaximumBuffers(&self, buffers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MaximumBuffers)(windows_core::Interface::as_raw(self), buffers) }
    }
    pub unsafe fn get_MinimumBuffers(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MinimumBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MinimumBuffers(&self, buffers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MinimumBuffers)(windows_core::Interface::as_raw(self), buffers) }
    }
    pub unsafe fn get_NumberOfBuffers(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NumberOfBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_NumberOfBuffers(&self, buffers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_NumberOfBuffers)(windows_core::Interface::as_raw(self), buffers) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_PreallocateFile(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PreallocateFile)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_PreallocateFile(&self, allocate: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PreallocateFile)(windows_core::Interface::as_raw(self), allocate) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_ProcessMode(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProcessMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_ProcessMode(&self, process: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ProcessMode)(windows_core::Interface::as_raw(self), process) }
    }
    pub unsafe fn get_RealTimeBuffersLost(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RealTimeBuffersLost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_RealTimeBuffersLost(&self, buffers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RealTimeBuffersLost)(windows_core::Interface::as_raw(self), buffers) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_SessionId(&self) -> windows_core::Result<super::ULONG64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SessionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_SessionId(&self, id: super::ULONG64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SessionId)(windows_core::Interface::as_raw(self), id) }
    }
    pub unsafe fn get_SessionName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SessionName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_SessionName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SessionName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn get_SessionThreadId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SessionThreadId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_SessionThreadId(&self, tid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SessionThreadId)(windows_core::Interface::as_raw(self), tid) }
    }
    pub unsafe fn get_StreamMode(&self) -> windows_core::Result<StreamMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StreamMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_StreamMode(&self, mode: StreamMode) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StreamMode)(windows_core::Interface::as_raw(self), mode) }
    }
    pub unsafe fn get_TraceDataProviders(&self) -> windows_core::Result<ITraceDataProviderCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TraceDataProviders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    pub get_BufferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_BufferSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_BuffersLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_BuffersLost: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_BuffersWritten: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_BuffersWritten: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_ClockType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClockType) -> windows_core::HRESULT,
    pub put_ClockType: unsafe extern "system" fn(*mut core::ffi::c_void, ClockType) -> windows_core::HRESULT,
    pub get_EventsLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_EventsLost: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_ExtendedModes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_ExtendedModes: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_FlushTimer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_FlushTimer: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_FreeBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_FreeBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_Guid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub put_Guid: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsKernelTrace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsKernelTrace: usize,
    pub get_MaximumBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_MaximumBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_MinimumBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_MinimumBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_NumberOfBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_NumberOfBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_PreallocateFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_PreallocateFile: usize,
    #[cfg(feature = "wtypes")]
    pub put_PreallocateFile: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_PreallocateFile: usize,
    #[cfg(feature = "wtypes")]
    pub get_ProcessMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_ProcessMode: usize,
    #[cfg(feature = "wtypes")]
    pub put_ProcessMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_ProcessMode: usize,
    pub get_RealTimeBuffersLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_RealTimeBuffersLost: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG64) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_SessionId: usize,
    #[cfg(feature = "shared")]
    pub put_SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG64) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_SessionId: usize,
    pub get_SessionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_SessionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SessionThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_SessionThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_StreamMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StreamMode) -> windows_core::HRESULT,
    pub put_StreamMode: unsafe extern "system" fn(*mut core::ffi::c_void, StreamMode) -> windows_core::HRESULT,
    pub get_TraceDataProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITraceDataCollector_Impl: IDataCollector_Impl {
    fn get_BufferSize(&self) -> windows_core::Result<u32>;
    fn put_BufferSize(&self, size: u32) -> windows_core::Result<()>;
    fn get_BuffersLost(&self) -> windows_core::Result<u32>;
    fn put_BuffersLost(&self, buffers: u32) -> windows_core::Result<()>;
    fn get_BuffersWritten(&self) -> windows_core::Result<u32>;
    fn put_BuffersWritten(&self, buffers: u32) -> windows_core::Result<()>;
    fn get_ClockType(&self) -> windows_core::Result<ClockType>;
    fn put_ClockType(&self, clock: ClockType) -> windows_core::Result<()>;
    fn get_EventsLost(&self) -> windows_core::Result<u32>;
    fn put_EventsLost(&self, events: u32) -> windows_core::Result<()>;
    fn get_ExtendedModes(&self) -> windows_core::Result<u32>;
    fn put_ExtendedModes(&self, mode: u32) -> windows_core::Result<()>;
    fn get_FlushTimer(&self) -> windows_core::Result<u32>;
    fn put_FlushTimer(&self, seconds: u32) -> windows_core::Result<()>;
    fn get_FreeBuffers(&self) -> windows_core::Result<u32>;
    fn put_FreeBuffers(&self, buffers: u32) -> windows_core::Result<()>;
    fn get_Guid(&self) -> windows_core::Result<windows_core::GUID>;
    fn put_Guid(&self, guid: &windows_core::GUID) -> windows_core::Result<()>;
    fn get_IsKernelTrace(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_MaximumBuffers(&self) -> windows_core::Result<u32>;
    fn put_MaximumBuffers(&self, buffers: u32) -> windows_core::Result<()>;
    fn get_MinimumBuffers(&self) -> windows_core::Result<u32>;
    fn put_MinimumBuffers(&self, buffers: u32) -> windows_core::Result<()>;
    fn get_NumberOfBuffers(&self) -> windows_core::Result<u32>;
    fn put_NumberOfBuffers(&self, buffers: u32) -> windows_core::Result<()>;
    fn get_PreallocateFile(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_PreallocateFile(&self, allocate: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_ProcessMode(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_ProcessMode(&self, process: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_RealTimeBuffersLost(&self) -> windows_core::Result<u32>;
    fn put_RealTimeBuffersLost(&self, buffers: u32) -> windows_core::Result<()>;
    fn get_SessionId(&self) -> windows_core::Result<super::ULONG64>;
    fn put_SessionId(&self, id: super::ULONG64) -> windows_core::Result<()>;
    fn get_SessionName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_SessionName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_SessionThreadId(&self) -> windows_core::Result<u32>;
    fn put_SessionThreadId(&self, tid: u32) -> windows_core::Result<()>;
    fn get_StreamMode(&self) -> windows_core::Result<StreamMode>;
    fn put_StreamMode(&self, mode: StreamMode) -> windows_core::Result<()>;
    fn get_TraceDataProviders(&self) -> windows_core::Result<ITraceDataProviderCollection>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITraceDataCollector_Vtbl {
    pub const fn new<Identity: ITraceDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_BufferSize<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_BufferSize(this) {
                    Ok(ok__) => {
                        size.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BufferSize<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_BufferSize(this, core::mem::transmute_copy(&size)).into()
            }
        }
        unsafe extern "system" fn get_BuffersLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_BuffersLost(this) {
                    Ok(ok__) => {
                        buffers.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BuffersLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_BuffersLost(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn get_BuffersWritten<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_BuffersWritten(this) {
                    Ok(ok__) => {
                        buffers.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BuffersWritten<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_BuffersWritten(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn get_ClockType<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clock: *mut ClockType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_ClockType(this) {
                    Ok(ok__) => {
                        clock.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ClockType<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clock: ClockType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_ClockType(this, core::mem::transmute_copy(&clock)).into()
            }
        }
        unsafe extern "system" fn get_EventsLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, events: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_EventsLost(this) {
                    Ok(ok__) => {
                        events.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_EventsLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, events: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_EventsLost(this, core::mem::transmute_copy(&events)).into()
            }
        }
        unsafe extern "system" fn get_ExtendedModes<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_ExtendedModes(this) {
                    Ok(ok__) => {
                        mode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ExtendedModes<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_ExtendedModes(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn get_FlushTimer<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_FlushTimer(this) {
                    Ok(ok__) => {
                        seconds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FlushTimer<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_FlushTimer(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn get_FreeBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_FreeBuffers(this) {
                    Ok(ok__) => {
                        buffers.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FreeBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_FreeBuffers(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn get_Guid<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_Guid(this) {
                    Ok(ok__) => {
                        guid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Guid<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_Guid(this, core::mem::transmute(&guid)).into()
            }
        }
        unsafe extern "system" fn get_IsKernelTrace<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kernel: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_IsKernelTrace(this) {
                    Ok(ok__) => {
                        kernel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_MaximumBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_MaximumBuffers(this) {
                    Ok(ok__) => {
                        buffers.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MaximumBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_MaximumBuffers(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn get_MinimumBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_MinimumBuffers(this) {
                    Ok(ok__) => {
                        buffers.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MinimumBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_MinimumBuffers(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn get_NumberOfBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_NumberOfBuffers(this) {
                    Ok(ok__) => {
                        buffers.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NumberOfBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_NumberOfBuffers(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn get_PreallocateFile<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allocate: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_PreallocateFile(this) {
                    Ok(ok__) => {
                        allocate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PreallocateFile<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allocate: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_PreallocateFile(this, core::mem::transmute_copy(&allocate)).into()
            }
        }
        unsafe extern "system" fn get_ProcessMode<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, process: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_ProcessMode(this) {
                    Ok(ok__) => {
                        process.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ProcessMode<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, process: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_ProcessMode(this, core::mem::transmute_copy(&process)).into()
            }
        }
        unsafe extern "system" fn get_RealTimeBuffersLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_RealTimeBuffersLost(this) {
                    Ok(ok__) => {
                        buffers.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RealTimeBuffersLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_RealTimeBuffersLost(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn get_SessionId<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut super::ULONG64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_SessionId(this) {
                    Ok(ok__) => {
                        id.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SessionId<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: super::ULONG64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_SessionId(this, core::mem::transmute_copy(&id)).into()
            }
        }
        unsafe extern "system" fn get_SessionName<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_SessionName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SessionName<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_SessionName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn get_SessionThreadId<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_SessionThreadId(this) {
                    Ok(ok__) => {
                        tid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SessionThreadId<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_SessionThreadId(this, core::mem::transmute_copy(&tid)).into()
            }
        }
        unsafe extern "system" fn get_StreamMode<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut StreamMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_StreamMode(this) {
                    Ok(ok__) => {
                        mode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StreamMode<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: StreamMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::put_StreamMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn get_TraceDataProviders<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::get_TraceDataProviders(this) {
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
            get_BufferSize: get_BufferSize::<Identity, OFFSET>,
            put_BufferSize: put_BufferSize::<Identity, OFFSET>,
            get_BuffersLost: get_BuffersLost::<Identity, OFFSET>,
            put_BuffersLost: put_BuffersLost::<Identity, OFFSET>,
            get_BuffersWritten: get_BuffersWritten::<Identity, OFFSET>,
            put_BuffersWritten: put_BuffersWritten::<Identity, OFFSET>,
            get_ClockType: get_ClockType::<Identity, OFFSET>,
            put_ClockType: put_ClockType::<Identity, OFFSET>,
            get_EventsLost: get_EventsLost::<Identity, OFFSET>,
            put_EventsLost: put_EventsLost::<Identity, OFFSET>,
            get_ExtendedModes: get_ExtendedModes::<Identity, OFFSET>,
            put_ExtendedModes: put_ExtendedModes::<Identity, OFFSET>,
            get_FlushTimer: get_FlushTimer::<Identity, OFFSET>,
            put_FlushTimer: put_FlushTimer::<Identity, OFFSET>,
            get_FreeBuffers: get_FreeBuffers::<Identity, OFFSET>,
            put_FreeBuffers: put_FreeBuffers::<Identity, OFFSET>,
            get_Guid: get_Guid::<Identity, OFFSET>,
            put_Guid: put_Guid::<Identity, OFFSET>,
            get_IsKernelTrace: get_IsKernelTrace::<Identity, OFFSET>,
            get_MaximumBuffers: get_MaximumBuffers::<Identity, OFFSET>,
            put_MaximumBuffers: put_MaximumBuffers::<Identity, OFFSET>,
            get_MinimumBuffers: get_MinimumBuffers::<Identity, OFFSET>,
            put_MinimumBuffers: put_MinimumBuffers::<Identity, OFFSET>,
            get_NumberOfBuffers: get_NumberOfBuffers::<Identity, OFFSET>,
            put_NumberOfBuffers: put_NumberOfBuffers::<Identity, OFFSET>,
            get_PreallocateFile: get_PreallocateFile::<Identity, OFFSET>,
            put_PreallocateFile: put_PreallocateFile::<Identity, OFFSET>,
            get_ProcessMode: get_ProcessMode::<Identity, OFFSET>,
            put_ProcessMode: put_ProcessMode::<Identity, OFFSET>,
            get_RealTimeBuffersLost: get_RealTimeBuffersLost::<Identity, OFFSET>,
            put_RealTimeBuffersLost: put_RealTimeBuffersLost::<Identity, OFFSET>,
            get_SessionId: get_SessionId::<Identity, OFFSET>,
            put_SessionId: put_SessionId::<Identity, OFFSET>,
            get_SessionName: get_SessionName::<Identity, OFFSET>,
            put_SessionName: put_SessionName::<Identity, OFFSET>,
            get_SessionThreadId: get_SessionThreadId::<Identity, OFFSET>,
            put_SessionThreadId: put_SessionThreadId::<Identity, OFFSET>,
            get_StreamMode: get_StreamMode::<Identity, OFFSET>,
            put_StreamMode: put_StreamMode::<Identity, OFFSET>,
            get_TraceDataProviders: get_TraceDataProviders::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceDataCollector as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITraceDataCollector {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITraceDataProvider, ITraceDataProvider_Vtbl, 0x03837512_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITraceDataProvider {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITraceDataProvider, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ITraceDataProvider {
    pub unsafe fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_DisplayName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn get_Guid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Guid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Guid(&self, guid: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Guid)(windows_core::Interface::as_raw(self), guid) }
    }
    pub unsafe fn get_Level(&self) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Level)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_KeywordsAny(&self) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_KeywordsAny)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_KeywordsAll(&self) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_KeywordsAll)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Properties(&self) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_FilterEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FilterEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_FilterEnabled(&self, filterenabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FilterEnabled)(windows_core::Interface::as_raw(self), filterenabled) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_FilterType(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FilterType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_FilterType(&self, ultype: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FilterType)(windows_core::Interface::as_raw(self), ultype) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_FilterData(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FilterData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_FilterData(&self, pdata: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FilterData)(windows_core::Interface::as_raw(self), pdata) }
    }
    pub unsafe fn Query(&self, bstrname: &windows_core::BSTR, bstrserver: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrserver)) }
    }
    pub unsafe fn Resolve<P0>(&self, pfrom: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), pfrom.param().abi()) }
    }
    pub unsafe fn SetSecurity(&self, sddl: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSecurity)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sddl)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSecurity(&self, securityinfo: super::ULONG) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurity)(windows_core::Interface::as_raw(self), securityinfo, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetRegisteredProcesses(&self) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRegisteredProcesses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataProvider_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Guid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub put_Guid: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub get_Level: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_KeywordsAny: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_KeywordsAll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_FilterEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_FilterEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_FilterEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_FilterEnabled: usize,
    #[cfg(feature = "shared")]
    pub get_FilterType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_FilterType: usize,
    #[cfg(feature = "shared")]
    pub put_FilterType: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_FilterType: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_FilterData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_FilterData: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_FilterData: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_FilterData: usize,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSecurity: usize,
    pub GetRegisteredProcesses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITraceDataProvider_Impl: super::IDispatch_Impl {
    fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_DisplayName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Guid(&self) -> windows_core::Result<windows_core::GUID>;
    fn put_Guid(&self, guid: &windows_core::GUID) -> windows_core::Result<()>;
    fn get_Level(&self) -> windows_core::Result<IValueMap>;
    fn get_KeywordsAny(&self) -> windows_core::Result<IValueMap>;
    fn get_KeywordsAll(&self) -> windows_core::Result<IValueMap>;
    fn get_Properties(&self) -> windows_core::Result<IValueMap>;
    fn get_FilterEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_FilterEnabled(&self, filterenabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_FilterType(&self) -> windows_core::Result<super::ULONG>;
    fn put_FilterType(&self, ultype: super::ULONG) -> windows_core::Result<()>;
    fn get_FilterData(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_FilterData(&self, pdata: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn Query(&self, bstrname: &windows_core::BSTR, bstrserver: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Resolve(&self, pfrom: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn SetSecurity(&self, sddl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetSecurity(&self, securityinfo: super::ULONG) -> windows_core::Result<windows_core::BSTR>;
    fn GetRegisteredProcesses(&self) -> windows_core::Result<IValueMap>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITraceDataProvider_Vtbl {
    pub const fn new<Identity: ITraceDataProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DisplayName<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::get_DisplayName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DisplayName<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::put_DisplayName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn get_Guid<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::get_Guid(this) {
                    Ok(ok__) => {
                        guid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Guid<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::put_Guid(this, core::mem::transmute(&guid)).into()
            }
        }
        unsafe extern "system" fn get_Level<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplevel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::get_Level(this) {
                    Ok(ok__) => {
                        pplevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_KeywordsAny<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppkeywords: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::get_KeywordsAny(this) {
                    Ok(ok__) => {
                        ppkeywords.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_KeywordsAll<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppkeywords: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::get_KeywordsAll(this) {
                    Ok(ok__) => {
                        ppkeywords.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Properties<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::get_Properties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FilterEnabled<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filterenabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::get_FilterEnabled(this) {
                    Ok(ok__) => {
                        filterenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FilterEnabled<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filterenabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::put_FilterEnabled(this, core::mem::transmute_copy(&filterenabled)).into()
            }
        }
        unsafe extern "system" fn get_FilterType<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pultype: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::get_FilterType(this) {
                    Ok(ok__) => {
                        pultype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FilterType<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ultype: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::put_FilterType(this, core::mem::transmute_copy(&ultype)).into()
            }
        }
        unsafe extern "system" fn get_FilterData<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdata: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::get_FilterData(this) {
                    Ok(ok__) => {
                        ppdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FilterData<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::put_FilterData(this, core::mem::transmute_copy(&pdata)).into()
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
        unsafe extern "system" fn GetSecurity<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, securityinfo: super::ULONG, sddl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_DisplayName: get_DisplayName::<Identity, OFFSET>,
            put_DisplayName: put_DisplayName::<Identity, OFFSET>,
            get_Guid: get_Guid::<Identity, OFFSET>,
            put_Guid: put_Guid::<Identity, OFFSET>,
            get_Level: get_Level::<Identity, OFFSET>,
            get_KeywordsAny: get_KeywordsAny::<Identity, OFFSET>,
            get_KeywordsAll: get_KeywordsAll::<Identity, OFFSET>,
            get_Properties: get_Properties::<Identity, OFFSET>,
            get_FilterEnabled: get_FilterEnabled::<Identity, OFFSET>,
            put_FilterEnabled: put_FilterEnabled::<Identity, OFFSET>,
            get_FilterType: get_FilterType::<Identity, OFFSET>,
            put_FilterType: put_FilterType::<Identity, OFFSET>,
            get_FilterData: get_FilterData::<Identity, OFFSET>,
            put_FilterData: put_FilterData::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            Resolve: Resolve::<Identity, OFFSET>,
            SetSecurity: SetSecurity::<Identity, OFFSET>,
            GetSecurity: GetSecurity::<Identity, OFFSET>,
            GetRegisteredProcesses: GetRegisteredProcesses::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceDataProvider as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITraceDataProvider {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITraceDataProviderCollection, ITraceDataProviderCollection_Vtbl, 0x03837510_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITraceDataProviderCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITraceDataProviderCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ITraceDataProviderCollection {
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<ITraceDataProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, pprovider: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITraceDataProvider>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pprovider.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Remove(&self, vprovider: &super::VARIANT) -> windows_core::HRESULT {
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
            (windows_core::Interface::vtable(self).CreateTraceDataProvider)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTraceDataProviders(&self, server: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTraceDataProviders)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(server)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetTraceDataProvidersByProcess(&self, server: &windows_core::BSTR, pid: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTraceDataProvidersByProcess)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(server), pid) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataProviderCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTraceDataProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTraceDataProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetTraceDataProvidersByProcess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTraceDataProvidersByProcess: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITraceDataProviderCollection_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<ITraceDataProvider>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pprovider: windows_core::Ref<ITraceDataProvider>) -> windows_core::Result<()>;
    fn Remove(&self, vprovider: super::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, providers: windows_core::Ref<ITraceDataProviderCollection>) -> windows_core::Result<()>;
    fn CreateTraceDataProvider(&self) -> windows_core::Result<ITraceDataProvider>;
    fn GetTraceDataProviders(&self, server: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetTraceDataProvidersByProcess(&self, server: &windows_core::BSTR, pid: super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITraceDataProviderCollection_Vtbl {
    pub const fn new<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProviderCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::VARIANT, ppprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProviderCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        ppprovider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProviderCollection_Impl::get__NewEnum(this) {
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
        unsafe extern "system" fn Remove<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vprovider: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProviderCollection_Impl::Remove(this, core::mem::transmute_copy(&vprovider)).into()
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
        unsafe extern "system" fn GetTraceDataProvidersByProcess<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: *mut core::ffi::c_void, pid: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProviderCollection_Impl::GetTraceDataProvidersByProcess(this, core::mem::transmute(&server), core::mem::transmute_copy(&pid)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Count: get_Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
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
        iid == &<ITraceDataProviderCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITraceDataProviderCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IValueMap, IValueMap_Vtbl, 0x03837534_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IValueMap {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IValueMap, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IValueMap {
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<IValueMapItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Description(&self, description: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Value(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Value(&self, value: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Value)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn get_ValueMapType(&self) -> windows_core::Result<ValueMapType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ValueMapType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ValueMapType(&self, r#type: ValueMapType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ValueMapType)(windows_core::Interface::as_raw(self), r#type) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Add(&self, value: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Remove(&self, value: &super::VARIANT) -> windows_core::HRESULT {
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
            (windows_core::Interface::vtable(self).CreateValueMapItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IValueMap_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Value: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Value: usize,
    pub get_ValueMapType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ValueMapType) -> windows_core::HRESULT,
    pub put_ValueMapType: unsafe extern "system" fn(*mut core::ffi::c_void, ValueMapType) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Add: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateValueMapItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IValueMap_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<IValueMapItem>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Description(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Value(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Value(&self, value: &super::VARIANT) -> windows_core::Result<()>;
    fn get_ValueMapType(&self) -> windows_core::Result<ValueMapType>;
    fn put_ValueMapType(&self, r#type: ValueMapType) -> windows_core::Result<()>;
    fn Add(&self, value: super::VARIANT) -> windows_core::Result<()>;
    fn Remove(&self, value: super::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, map: windows_core::Ref<IValueMap>) -> windows_core::Result<()>;
    fn CreateValueMapItem(&self) -> windows_core::Result<IValueMapItem>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IValueMap_Vtbl {
    pub const fn new<Identity: IValueMap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::get_Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::VARIANT, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Description<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::get_Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Description<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::put_Description(this, core::mem::transmute(&description)).into()
            }
        }
        unsafe extern "system" fn get_Value<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::get_Value(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Value<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::put_Value(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get_ValueMapType<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut ValueMapType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::get_ValueMapType(this) {
                    Ok(ok__) => {
                        r#type.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ValueMapType<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: ValueMapType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::put_ValueMapType(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::Add(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::Remove(this, core::mem::transmute_copy(&value)).into()
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Count: get_Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Description: get_Description::<Identity, OFFSET>,
            put_Description: put_Description::<Identity, OFFSET>,
            get_Value: get_Value::<Identity, OFFSET>,
            put_Value: put_Value::<Identity, OFFSET>,
            get_ValueMapType: get_ValueMapType::<Identity, OFFSET>,
            put_ValueMapType: put_ValueMapType::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateValueMapItem: CreateValueMapItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IValueMap as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IValueMap {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IValueMapItem, IValueMapItem_Vtbl, 0x03837533_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IValueMapItem {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IValueMapItem, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IValueMapItem {
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Description(&self, description: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Enabled)(windows_core::Interface::as_raw(self), enabled) }
    }
    pub unsafe fn get_Key(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Key)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Key(&self, key: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Key)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(key)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Value(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Value(&self, value: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Value)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn get_ValueMapType(&self) -> windows_core::Result<ValueMapType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ValueMapType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ValueMapType(&self, r#type: ValueMapType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ValueMapType)(windows_core::Interface::as_raw(self), r#type) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IValueMapItem_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Enabled: usize,
    pub get_Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Value: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Value: usize,
    pub get_ValueMapType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ValueMapType) -> windows_core::HRESULT,
    pub put_ValueMapType: unsafe extern "system" fn(*mut core::ffi::c_void, ValueMapType) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IValueMapItem_Impl: super::IDispatch_Impl {
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Description(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Key(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Key(&self, key: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Value(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Value(&self, value: &super::VARIANT) -> windows_core::Result<()>;
    fn get_ValueMapType(&self) -> windows_core::Result<ValueMapType>;
    fn put_ValueMapType(&self, r#type: ValueMapType) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IValueMapItem_Vtbl {
    pub const fn new<Identity: IValueMapItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Description<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMapItem_Impl::get_Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Description<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMapItem_Impl::put_Description(this, core::mem::transmute(&description)).into()
            }
        }
        unsafe extern "system" fn get_Enabled<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMapItem_Impl::get_Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Enabled<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMapItem_Impl::put_Enabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn get_Key<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMapItem_Impl::get_Key(this) {
                    Ok(ok__) => {
                        key.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Key<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMapItem_Impl::put_Key(this, core::mem::transmute(&key)).into()
            }
        }
        unsafe extern "system" fn get_Value<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMapItem_Impl::get_Value(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Value<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMapItem_Impl::put_Value(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get_ValueMapType<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut ValueMapType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMapItem_Impl::get_ValueMapType(this) {
                    Ok(ok__) => {
                        r#type.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ValueMapType<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: ValueMapType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMapItem_Impl::put_ValueMapType(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Description: get_Description::<Identity, OFFSET>,
            put_Description: put_Description::<Identity, OFFSET>,
            get_Enabled: get_Enabled::<Identity, OFFSET>,
            put_Enabled: put_Enabled::<Identity, OFFSET>,
            get_Key: get_Key::<Identity, OFFSET>,
            put_Key: put_Key::<Identity, OFFSET>,
            get_Value: get_Value::<Identity, OFFSET>,
            put_Value: put_Value::<Identity, OFFSET>,
            get_ValueMapType: get_ValueMapType::<Identity, OFFSET>,
            put_ValueMapType: put_ValueMapType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IValueMapItem as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IValueMapItem {}
pub const LegacyDataCollectorSet: windows_core::GUID = windows_core::GUID::from_u128(0x03837526_098b_11d8_9414_505054503030);
pub const LegacyDataCollectorSetCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837527_098b_11d8_9414_505054503030);
pub const LegacyTraceSession: windows_core::GUID = windows_core::GUID::from_u128(0x03837528_098b_11d8_9414_505054503030);
pub const LegacyTraceSessionCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837529_098b_11d8_9414_505054503030);
#[cfg(feature = "shared")]
pub const PLAL_ALERT_CMD_LINE_A_NAME: super::DWORD = 512;
#[cfg(feature = "shared")]
pub const PLAL_ALERT_CMD_LINE_C_NAME: super::DWORD = 1024;
#[cfg(feature = "shared")]
pub const PLAL_ALERT_CMD_LINE_D_TIME: super::DWORD = 2048;
#[cfg(feature = "shared")]
pub const PLAL_ALERT_CMD_LINE_L_VAL: super::DWORD = 4096;
#[cfg(feature = "shared")]
pub const PLAL_ALERT_CMD_LINE_MASK: super::DWORD = 32512;
#[cfg(feature = "shared")]
pub const PLAL_ALERT_CMD_LINE_M_VAL: super::DWORD = 8192;
#[cfg(feature = "shared")]
pub const PLAL_ALERT_CMD_LINE_SINGLE: super::DWORD = 256;
#[cfg(feature = "shared")]
pub const PLAL_ALERT_CMD_LINE_U_TEXT: super::DWORD = 16384;
#[cfg(feature = "winnt")]
pub type PLA_CABEXTRACT_CALLBACK = Option<unsafe extern "C" fn(filename: windows_core::PCWSTR, context: super::PVOID)>;
pub const PLA_CAPABILITY_AUTOLOGGER: i32 = 32;
pub const PLA_CAPABILITY_LEGACY_SESSION: i32 = 8;
pub const PLA_CAPABILITY_LEGACY_SVC: i32 = 16;
pub const PLA_CAPABILITY_LOCAL: i32 = 268435456;
pub const PLA_CAPABILITY_V1_SESSION: i32 = 2;
pub const PLA_CAPABILITY_V1_SVC: i32 = 1;
pub const PLA_CAPABILITY_V1_SYSTEM: i32 = 4;
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
