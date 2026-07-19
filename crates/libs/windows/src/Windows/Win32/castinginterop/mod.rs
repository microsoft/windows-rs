pub type CASTING_CONNECTION_ERROR_STATUS = i32;
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_DID_NOT_RESPOND: CASTING_CONNECTION_ERROR_STATUS = 1;
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_ERROR: CASTING_CONNECTION_ERROR_STATUS = 2;
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_LOCKED: CASTING_CONNECTION_ERROR_STATUS = 3;
pub const CASTING_CONNECTION_ERROR_STATUS_INVALID_CASTING_SOURCE: CASTING_CONNECTION_ERROR_STATUS = 5;
pub const CASTING_CONNECTION_ERROR_STATUS_PROTECTED_PLAYBACK_FAILED: CASTING_CONNECTION_ERROR_STATUS = 4;
pub const CASTING_CONNECTION_ERROR_STATUS_SUCCEEDED: CASTING_CONNECTION_ERROR_STATUS = 0;
pub const CASTING_CONNECTION_ERROR_STATUS_UNKNOWN: CASTING_CONNECTION_ERROR_STATUS = 6;
pub type CASTING_CONNECTION_STATE = i32;
pub const CASTING_CONNECTION_STATE_CONNECTED: CASTING_CONNECTION_STATE = 1;
pub const CASTING_CONNECTION_STATE_CONNECTING: CASTING_CONNECTION_STATE = 4;
pub const CASTING_CONNECTION_STATE_DISCONNECTED: CASTING_CONNECTION_STATE = 0;
pub const CASTING_CONNECTION_STATE_DISCONNECTING: CASTING_CONNECTION_STATE = 3;
pub const CASTING_CONNECTION_STATE_RENDERING: CASTING_CONNECTION_STATE = 2;
windows_core::imp::define_interface!(ICastingController, ICastingController_Vtbl, 0xf0a56423_a664_4fbd_8b43_409a45e8d9a1);
windows_core::imp::interface_hierarchy!(ICastingController, windows_core::IUnknown);
impl ICastingController {
    pub unsafe fn Initialize<P0, P1>(&self, castingengine: P0, castingsource: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), castingengine.param().abi(), castingsource.param().abi()) }
    }
    pub unsafe fn Connect(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Disconnect(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Advise<P0>(&self, eventhandler: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<ICastingEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnAdvise(&self, cookie: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnAdvise)(windows_core::Interface::as_raw(self), cookie) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingController_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ICastingController_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, castingengine: windows_core::Ref<windows_core::IUnknown>, castingsource: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Connect(&self) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn Advise(&self, eventhandler: windows_core::Ref<ICastingEventHandler>) -> windows_core::Result<u32>;
    fn UnAdvise(&self, cookie: u32) -> windows_core::Result<()>;
}
impl ICastingController_Vtbl {
    pub const fn new<Identity: ICastingController_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: ICastingController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, castingengine: *mut core::ffi::c_void, castingsource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICastingController_Impl::Initialize(this, core::mem::transmute_copy(&castingengine), core::mem::transmute_copy(&castingsource)).into()
            }
        }
        unsafe extern "system" fn Connect<Identity: ICastingController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICastingController_Impl::Connect(this).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: ICastingController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICastingController_Impl::Disconnect(this).into()
            }
        }
        unsafe extern "system" fn Advise<Identity: ICastingController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventhandler: *mut core::ffi::c_void, cookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICastingController_Impl::Advise(this, core::mem::transmute_copy(&eventhandler)) {
                    Ok(ok__) => {
                        cookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnAdvise<Identity: ICastingController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICastingController_Impl::UnAdvise(this, core::mem::transmute_copy(&cookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            UnAdvise: UnAdvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICastingController as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICastingController {}
windows_core::imp::define_interface!(ICastingEventHandler, ICastingEventHandler_Vtbl, 0xc79a6cb7_bebd_47a6_a2ad_4d45ad79c7bc);
windows_core::imp::interface_hierarchy!(ICastingEventHandler, windows_core::IUnknown);
impl ICastingEventHandler {
    pub unsafe fn OnStateChanged(&self, newstate: CASTING_CONNECTION_STATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStateChanged)(windows_core::Interface::as_raw(self), newstate) }
    }
    pub unsafe fn OnError<P1>(&self, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), errorstatus, errormessage.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, CASTING_CONNECTION_STATE) -> windows_core::HRESULT,
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, CASTING_CONNECTION_ERROR_STATUS, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait ICastingEventHandler_Impl: windows_core::IUnknownImpl {
    fn OnStateChanged(&self, newstate: CASTING_CONNECTION_STATE) -> windows_core::Result<()>;
    fn OnError(&self, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl ICastingEventHandler_Vtbl {
    pub const fn new<Identity: ICastingEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStateChanged<Identity: ICastingEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstate: CASTING_CONNECTION_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICastingEventHandler_Impl::OnStateChanged(this, core::mem::transmute_copy(&newstate)).into()
            }
        }
        unsafe extern "system" fn OnError<Identity: ICastingEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICastingEventHandler_Impl::OnError(this, core::mem::transmute_copy(&errorstatus), core::mem::transmute(&errormessage)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStateChanged: OnStateChanged::<Identity, OFFSET>,
            OnError: OnError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICastingEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICastingEventHandler {}
windows_core::imp::define_interface!(ICastingSourceInfo, ICastingSourceInfo_Vtbl, 0x45101ab7_7c3a_4bce_9500_12c09024b298);
windows_core::imp::interface_hierarchy!(ICastingSourceInfo, windows_core::IUnknown);
impl ICastingSourceInfo {
    pub unsafe fn GetController(&self) -> windows_core::Result<ICastingController> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetController)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "propsys")]
    pub unsafe fn GetProperties(&self) -> windows_core::Result<super::INamedPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingSourceInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "propsys")]
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    GetProperties: usize,
}
#[cfg(feature = "propsys")]
pub trait ICastingSourceInfo_Impl: windows_core::IUnknownImpl {
    fn GetController(&self) -> windows_core::Result<ICastingController>;
    fn GetProperties(&self) -> windows_core::Result<super::INamedPropertyStore>;
}
#[cfg(feature = "propsys")]
impl ICastingSourceInfo_Vtbl {
    pub const fn new<Identity: ICastingSourceInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetController<Identity: ICastingSourceInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, controller: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICastingSourceInfo_Impl::GetController(this) {
                    Ok(ok__) => {
                        controller.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperties<Identity: ICastingSourceInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, props: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICastingSourceInfo_Impl::GetProperties(this) {
                    Ok(ok__) => {
                        props.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetController: GetController::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICastingSourceInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "propsys")]
impl windows_core::RuntimeName for ICastingSourceInfo {}
