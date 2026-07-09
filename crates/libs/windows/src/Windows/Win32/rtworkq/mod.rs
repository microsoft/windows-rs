#[inline]
pub unsafe fn RtwqAddPeriodicCallback<P1>(callback: RTWQPERIODICCALLBACK, context: P1, key: Option<*mut u32>) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("rtworkq.dll" "system" fn RtwqAddPeriodicCallback(callback : RTWQPERIODICCALLBACK, context : *mut core::ffi::c_void, key : *mut u32) -> windows_core::HRESULT);
    unsafe { RtwqAddPeriodicCallback(callback, context.param().abi(), key.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RtwqAllocateSerialWorkQueue(workqueueidin: u32) -> windows_core::Result<u32> {
    windows_core::link!("rtworkq.dll" "system" fn RtwqAllocateSerialWorkQueue(workqueueidin : u32, workqueueidout : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RtwqAllocateSerialWorkQueue(workqueueidin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RtwqAllocateWorkQueue(workqueuetype: RTWQ_WORKQUEUE_TYPE) -> windows_core::Result<u32> {
    windows_core::link!("rtworkq.dll" "system" fn RtwqAllocateWorkQueue(workqueuetype : RTWQ_WORKQUEUE_TYPE, workqueueid : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RtwqAllocateWorkQueue(workqueuetype, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RtwqBeginRegisterWorkQueueWithMMCSS<P1, P4, P5>(workqueueid: u32, usageclass: P1, dwtaskid: u32, lpriority: i32, donecallback: P4, donestate: P5) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<IRtwqAsyncCallback>,
    P5: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("rtworkq.dll" "system" fn RtwqBeginRegisterWorkQueueWithMMCSS(workqueueid : u32, usageclass : windows_core::PCWSTR, dwtaskid : u32, lpriority : i32, donecallback : *mut core::ffi::c_void, donestate : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RtwqBeginRegisterWorkQueueWithMMCSS(workqueueid, usageclass.param().abi(), dwtaskid, lpriority, donecallback.param().abi(), donestate.param().abi()) }
}
#[inline]
pub unsafe fn RtwqBeginUnregisterWorkQueueWithMMCSS<P1, P2>(workqueueid: u32, donecallback: P1, donestate: P2) -> windows_core::HRESULT
where
    P1: windows_core::Param<IRtwqAsyncCallback>,
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("rtworkq.dll" "system" fn RtwqBeginUnregisterWorkQueueWithMMCSS(workqueueid : u32, donecallback : *mut core::ffi::c_void, donestate : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RtwqBeginUnregisterWorkQueueWithMMCSS(workqueueid, donecallback.param().abi(), donestate.param().abi()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn RtwqCancelDeadline(prequest: super::winnt::HANDLE) -> windows_core::HRESULT {
    windows_core::link!("rtworkq.dll" "system" fn RtwqCancelDeadline(prequest : super::winnt::HANDLE) -> windows_core::HRESULT);
    unsafe { RtwqCancelDeadline(prequest) }
}
#[inline]
pub unsafe fn RtwqCancelWorkItem(key: RTWQWORKITEM_KEY) -> windows_core::HRESULT {
    windows_core::link!("rtworkq.dll" "system" fn RtwqCancelWorkItem(key : RTWQWORKITEM_KEY) -> windows_core::HRESULT);
    unsafe { RtwqCancelWorkItem(key) }
}
#[inline]
pub unsafe fn RtwqCreateAsyncResult<P0, P1, P2>(appobject: P0, callback: P1, appstate: P2) -> windows_core::Result<IRtwqAsyncResult>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<IRtwqAsyncCallback>,
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("rtworkq.dll" "system" fn RtwqCreateAsyncResult(appobject : *mut core::ffi::c_void, callback : *mut core::ffi::c_void, appstate : *mut core::ffi::c_void, asyncresult : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RtwqCreateAsyncResult(appobject.param().abi(), callback.param().abi(), appstate.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn RtwqEndRegisterWorkQueueWithMMCSS<P0>(result: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<IRtwqAsyncResult>,
{
    windows_core::link!("rtworkq.dll" "system" fn RtwqEndRegisterWorkQueueWithMMCSS(result : *mut core::ffi::c_void, taskid : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RtwqEndRegisterWorkQueueWithMMCSS(result.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RtwqGetWorkQueueMMCSSClass(workqueueid: u32, usageclass: Option<windows_core::PWSTR>, usageclasslength: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("rtworkq.dll" "system" fn RtwqGetWorkQueueMMCSSClass(workqueueid : u32, usageclass : windows_core::PWSTR, usageclasslength : *mut u32) -> windows_core::HRESULT);
    unsafe { RtwqGetWorkQueueMMCSSClass(workqueueid, usageclass.unwrap_or(core::mem::zeroed()) as _, usageclasslength as _) }
}
#[inline]
pub unsafe fn RtwqGetWorkQueueMMCSSPriority(workqueueid: u32) -> windows_core::Result<i32> {
    windows_core::link!("rtworkq.dll" "system" fn RtwqGetWorkQueueMMCSSPriority(workqueueid : u32, priority : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RtwqGetWorkQueueMMCSSPriority(workqueueid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RtwqGetWorkQueueMMCSSTaskId(workqueueid: u32) -> windows_core::Result<u32> {
    windows_core::link!("rtworkq.dll" "system" fn RtwqGetWorkQueueMMCSSTaskId(workqueueid : u32, taskid : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RtwqGetWorkQueueMMCSSTaskId(workqueueid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RtwqInvokeCallback<P0>(result: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<IRtwqAsyncResult>,
{
    windows_core::link!("rtworkq.dll" "system" fn RtwqInvokeCallback(result : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RtwqInvokeCallback(result.param().abi()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn RtwqJoinWorkQueue(workqueueid: u32, hfile: super::winnt::HANDLE) -> windows_core::Result<super::winnt::HANDLE> {
    windows_core::link!("rtworkq.dll" "system" fn RtwqJoinWorkQueue(workqueueid : u32, hfile : super::winnt::HANDLE, out : *mut super::winnt::HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RtwqJoinWorkQueue(workqueueid, hfile, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RtwqLockPlatform() -> windows_core::HRESULT {
    windows_core::link!("rtworkq.dll" "system" fn RtwqLockPlatform() -> windows_core::HRESULT);
    unsafe { RtwqLockPlatform() }
}
#[inline]
pub unsafe fn RtwqLockSharedWorkQueue<P0>(usageclass: P0, basepriority: i32, taskid: *mut u32, id: *mut u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rtworkq.dll" "system" fn RtwqLockSharedWorkQueue(usageclass : windows_core::PCWSTR, basepriority : i32, taskid : *mut u32, id : *mut u32) -> windows_core::HRESULT);
    unsafe { RtwqLockSharedWorkQueue(usageclass.param().abi(), basepriority, taskid as _, id as _) }
}
#[inline]
pub unsafe fn RtwqLockWorkQueue(workqueueid: u32) -> windows_core::HRESULT {
    windows_core::link!("rtworkq.dll" "system" fn RtwqLockWorkQueue(workqueueid : u32) -> windows_core::HRESULT);
    unsafe { RtwqLockWorkQueue(workqueueid) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn RtwqPutWaitingWorkItem<P2>(hevent: super::winnt::HANDLE, lpriority: i32, result: P2, key: Option<*mut RTWQWORKITEM_KEY>) -> windows_core::HRESULT
where
    P2: windows_core::Param<IRtwqAsyncResult>,
{
    windows_core::link!("rtworkq.dll" "system" fn RtwqPutWaitingWorkItem(hevent : super::winnt::HANDLE, lpriority : i32, result : *mut core::ffi::c_void, key : *mut RTWQWORKITEM_KEY) -> windows_core::HRESULT);
    unsafe { RtwqPutWaitingWorkItem(hevent, lpriority, result.param().abi(), key.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RtwqPutWorkItem<P2>(dwqueue: u32, lpriority: i32, result: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<IRtwqAsyncResult>,
{
    windows_core::link!("rtworkq.dll" "system" fn RtwqPutWorkItem(dwqueue : u32, lpriority : i32, result : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RtwqPutWorkItem(dwqueue, lpriority, result.param().abi()) }
}
#[inline]
pub unsafe fn RtwqRegisterPlatformEvents<P0>(platformevents: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<IRtwqPlatformEvents>,
{
    windows_core::link!("rtworkq.dll" "system" fn RtwqRegisterPlatformEvents(platformevents : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RtwqRegisterPlatformEvents(platformevents.param().abi()) }
}
#[inline]
pub unsafe fn RtwqRegisterPlatformWithMMCSS<P0>(usageclass: P0, taskid: *mut u32, lpriority: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rtworkq.dll" "system" fn RtwqRegisterPlatformWithMMCSS(usageclass : windows_core::PCWSTR, taskid : *mut u32, lpriority : i32) -> windows_core::HRESULT);
    unsafe { RtwqRegisterPlatformWithMMCSS(usageclass.param().abi(), taskid as _, lpriority) }
}
#[inline]
pub unsafe fn RtwqRemovePeriodicCallback(dwkey: u32) -> windows_core::HRESULT {
    windows_core::link!("rtworkq.dll" "system" fn RtwqRemovePeriodicCallback(dwkey : u32) -> windows_core::HRESULT);
    unsafe { RtwqRemovePeriodicCallback(dwkey) }
}
#[inline]
pub unsafe fn RtwqScheduleWorkItem<P0>(result: P0, timeout: i64, key: Option<*mut RTWQWORKITEM_KEY>) -> windows_core::HRESULT
where
    P0: windows_core::Param<IRtwqAsyncResult>,
{
    windows_core::link!("rtworkq.dll" "system" fn RtwqScheduleWorkItem(result : *mut core::ffi::c_void, timeout : i64, key : *mut RTWQWORKITEM_KEY) -> windows_core::HRESULT);
    unsafe { RtwqScheduleWorkItem(result.param().abi(), timeout, key.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn RtwqSetDeadline(workqueueid: u32, deadlineinhns: i64) -> windows_core::Result<super::winnt::HANDLE> {
    windows_core::link!("rtworkq.dll" "system" fn RtwqSetDeadline(workqueueid : u32, deadlineinhns : i64, prequest : *mut super::winnt::HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RtwqSetDeadline(workqueueid, deadlineinhns, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn RtwqSetDeadline2(workqueueid: u32, deadlineinhns: i64, predeadlineinhns: i64) -> windows_core::Result<super::winnt::HANDLE> {
    windows_core::link!("rtworkq.dll" "system" fn RtwqSetDeadline2(workqueueid : u32, deadlineinhns : i64, predeadlineinhns : i64, prequest : *mut super::winnt::HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RtwqSetDeadline2(workqueueid, deadlineinhns, predeadlineinhns, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RtwqSetLongRunning(workqueueid: u32, enable: bool) -> windows_core::HRESULT {
    windows_core::link!("rtworkq.dll" "system" fn RtwqSetLongRunning(workqueueid : u32, enable : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { RtwqSetLongRunning(workqueueid, enable.into()) }
}
#[inline]
pub unsafe fn RtwqShutdown() -> windows_core::HRESULT {
    windows_core::link!("rtworkq.dll" "system" fn RtwqShutdown() -> windows_core::HRESULT);
    unsafe { RtwqShutdown() }
}
#[inline]
pub unsafe fn RtwqStartup() -> windows_core::HRESULT {
    windows_core::link!("rtworkq.dll" "system" fn RtwqStartup() -> windows_core::HRESULT);
    unsafe { RtwqStartup() }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn RtwqUnjoinWorkQueue(workqueueid: u32, hfile: super::winnt::HANDLE) -> windows_core::HRESULT {
    windows_core::link!("rtworkq.dll" "system" fn RtwqUnjoinWorkQueue(workqueueid : u32, hfile : super::winnt::HANDLE) -> windows_core::HRESULT);
    unsafe { RtwqUnjoinWorkQueue(workqueueid, hfile) }
}
#[inline]
pub unsafe fn RtwqUnlockPlatform() -> windows_core::HRESULT {
    windows_core::link!("rtworkq.dll" "system" fn RtwqUnlockPlatform() -> windows_core::HRESULT);
    unsafe { RtwqUnlockPlatform() }
}
#[inline]
pub unsafe fn RtwqUnlockWorkQueue(workqueueid: u32) -> windows_core::HRESULT {
    windows_core::link!("rtworkq.dll" "system" fn RtwqUnlockWorkQueue(workqueueid : u32) -> windows_core::HRESULT);
    unsafe { RtwqUnlockWorkQueue(workqueueid) }
}
#[inline]
pub unsafe fn RtwqUnregisterPlatformEvents<P0>(platformevents: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<IRtwqPlatformEvents>,
{
    windows_core::link!("rtworkq.dll" "system" fn RtwqUnregisterPlatformEvents(platformevents : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RtwqUnregisterPlatformEvents(platformevents.param().abi()) }
}
#[inline]
pub unsafe fn RtwqUnregisterPlatformFromMMCSS() -> windows_core::HRESULT {
    windows_core::link!("rtworkq.dll" "system" fn RtwqUnregisterPlatformFromMMCSS() -> windows_core::HRESULT);
    unsafe { RtwqUnregisterPlatformFromMMCSS() }
}
windows_core::imp::define_interface!(IRtwqAsyncCallback, IRtwqAsyncCallback_Vtbl, 0xa27003cf_2354_4f2a_8d6a_ab7cff15437e);
windows_core::imp::interface_hierarchy!(IRtwqAsyncCallback, windows_core::IUnknown);
impl IRtwqAsyncCallback {
    pub unsafe fn GetParameters(&self, pdwflags: *mut u32, pdwqueue: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetParameters)(windows_core::Interface::as_raw(self), pdwflags as _, pdwqueue as _) }
    }
    pub unsafe fn Invoke<P0>(&self, pasyncresult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IRtwqAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), pasyncresult.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRtwqAsyncCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRtwqAsyncCallback_Impl: windows_core::IUnknownImpl {
    fn GetParameters(&self, pdwflags: *mut u32, pdwqueue: *mut u32) -> windows_core::Result<()>;
    fn Invoke(&self, pasyncresult: windows_core::Ref<IRtwqAsyncResult>) -> windows_core::Result<()>;
}
impl IRtwqAsyncCallback_Vtbl {
    pub const fn new<Identity: IRtwqAsyncCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetParameters<Identity: IRtwqAsyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32, pdwqueue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRtwqAsyncCallback_Impl::GetParameters(this, core::mem::transmute_copy(&pdwflags), core::mem::transmute_copy(&pdwqueue)).into()
            }
        }
        unsafe extern "system" fn Invoke<Identity: IRtwqAsyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasyncresult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRtwqAsyncCallback_Impl::Invoke(this, core::mem::transmute_copy(&pasyncresult)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetParameters: GetParameters::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRtwqAsyncCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRtwqAsyncCallback {}
windows_core::imp::define_interface!(IRtwqAsyncResult, IRtwqAsyncResult_Vtbl, 0xac6b7889_0740_4d51_8619_905994a55cc6);
windows_core::imp::interface_hierarchy!(IRtwqAsyncResult, windows_core::IUnknown);
impl IRtwqAsyncResult {
    pub unsafe fn GetState(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetStatus(&self, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), hrstatus) }
    }
    pub unsafe fn GetObject(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStateNoAddRef(&self) -> Option<windows_core::IUnknown> {
        unsafe { (windows_core::Interface::vtable(self).GetStateNoAddRef)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRtwqAsyncResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStateNoAddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<windows_core::IUnknown>,
}
pub trait IRtwqAsyncResult_Impl: windows_core::IUnknownImpl {
    fn GetState(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetStatus(&self) -> windows_core::Result<()>;
    fn SetStatus(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetObject(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetStateNoAddRef(&self) -> Option<windows_core::IUnknown>;
}
impl IRtwqAsyncResult_Vtbl {
    pub const fn new<Identity: IRtwqAsyncResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetState<Identity: IRtwqAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRtwqAsyncResult_Impl::GetState(this) {
                    Ok(ok__) => {
                        ppunkstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IRtwqAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRtwqAsyncResult_Impl::GetStatus(this).into()
            }
        }
        unsafe extern "system" fn SetStatus<Identity: IRtwqAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRtwqAsyncResult_Impl::SetStatus(this, core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        unsafe extern "system" fn GetObject<Identity: IRtwqAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRtwqAsyncResult_Impl::GetObject(this) {
                    Ok(ok__) => {
                        ppobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStateNoAddRef<Identity: IRtwqAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> Option<windows_core::IUnknown> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRtwqAsyncResult_Impl::GetStateNoAddRef(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetState: GetState::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            SetStatus: SetStatus::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            GetStateNoAddRef: GetStateNoAddRef::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRtwqAsyncResult as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRtwqAsyncResult {}
windows_core::imp::define_interface!(IRtwqPlatformEvents, IRtwqPlatformEvents_Vtbl, 0x63d9255a_7ff1_4b61_8faf_ed6460dacf2b);
windows_core::imp::interface_hierarchy!(IRtwqPlatformEvents, windows_core::IUnknown);
impl IRtwqPlatformEvents {
    pub unsafe fn InitializationComplete(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializationComplete)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ShutdownStart(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShutdownStart)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ShutdownComplete(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShutdownComplete)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRtwqPlatformEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializationComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShutdownStart: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShutdownComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRtwqPlatformEvents_Impl: windows_core::IUnknownImpl {
    fn InitializationComplete(&self) -> windows_core::Result<()>;
    fn ShutdownStart(&self) -> windows_core::Result<()>;
    fn ShutdownComplete(&self) -> windows_core::Result<()>;
}
impl IRtwqPlatformEvents_Vtbl {
    pub const fn new<Identity: IRtwqPlatformEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializationComplete<Identity: IRtwqPlatformEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRtwqPlatformEvents_Impl::InitializationComplete(this).into()
            }
        }
        unsafe extern "system" fn ShutdownStart<Identity: IRtwqPlatformEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRtwqPlatformEvents_Impl::ShutdownStart(this).into()
            }
        }
        unsafe extern "system" fn ShutdownComplete<Identity: IRtwqPlatformEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRtwqPlatformEvents_Impl::ShutdownComplete(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializationComplete: InitializationComplete::<Identity, OFFSET>,
            ShutdownStart: ShutdownStart::<Identity, OFFSET>,
            ShutdownComplete: ShutdownComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRtwqPlatformEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRtwqPlatformEvents {}
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
pub struct RTWQASYNCRESULT {
    pub Base: core::mem::ManuallyDrop<Option<IRtwqAsyncResult>>,
    pub overlapped: super::minwinbase::OVERLAPPED,
    pub pCallback: core::mem::ManuallyDrop<Option<IRtwqAsyncCallback>>,
    pub hrStatusResult: windows_core::HRESULT,
    pub dwBytesTransferred: u32,
    pub hEvent: super::winnt::HANDLE,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
impl Clone for RTWQASYNCRESULT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
impl Default for RTWQASYNCRESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RTWQPERIODICCALLBACK = Option<unsafe extern "system" fn(context: windows_core::Ref<windows_core::IUnknown>)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RTWQWORKITEM_KEY(pub u64);
pub const RTWQ_E_BUFFERTOOSMALL: i32 = -1072875855;
pub const RTWQ_E_INVALID_WORKQUEUE: i32 = -1072875777;
pub const RTWQ_E_NOT_FOUND: i32 = -1072875819;
pub const RTWQ_E_NOT_INITIALIZED: i32 = -1072875850;
pub const RTWQ_E_OPERATION_CANCELLED: i32 = -1072875795;
pub const RTWQ_E_SHUTDOWN: i32 = -1072873851;
pub const RTWQ_E_UNEXPECTED: i32 = -1072875845;
pub const RTWQ_MULTITHREADED_WORKQUEUE: RTWQ_WORKQUEUE_TYPE = 2;
pub const RTWQ_STANDARD_WORKQUEUE: RTWQ_WORKQUEUE_TYPE = 0;
pub const RTWQ_WINDOW_WORKQUEUE: RTWQ_WORKQUEUE_TYPE = 1;
pub type RTWQ_WORKQUEUE_TYPE = i32;
