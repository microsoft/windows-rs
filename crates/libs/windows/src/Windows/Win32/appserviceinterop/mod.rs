windows_core::imp::define_interface!(IAppServiceConnectionExtendedExecution, IAppServiceConnectionExtendedExecution_Vtbl, 0x65219584_f9cb_4ae3_81f9_a28a6ca450d9);
windows_core::imp::interface_hierarchy!(IAppServiceConnectionExtendedExecution, windows_core::IUnknown);
impl IAppServiceConnectionExtendedExecution {
    pub unsafe fn OpenForExtendedExecutionAsync<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).OpenForExtendedExecutionAsync)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnectionExtendedExecution_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OpenForExtendedExecutionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppServiceConnectionExtendedExecution_Impl: windows_core::IUnknownImpl {
    fn OpenForExtendedExecutionAsync(&self, riid: *const windows_core::GUID, operation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IAppServiceConnectionExtendedExecution_Vtbl {
    pub const fn new<Identity: IAppServiceConnectionExtendedExecution_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenForExtendedExecutionAsync<Identity: IAppServiceConnectionExtendedExecution_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, operation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppServiceConnectionExtendedExecution_Impl::OpenForExtendedExecutionAsync(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&operation)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OpenForExtendedExecutionAsync: OpenForExtendedExecutionAsync::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppServiceConnectionExtendedExecution as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppServiceConnectionExtendedExecution {}
windows_core::imp::define_interface!(ICorrelationVectorSource, ICorrelationVectorSource_Vtbl, 0x152b8a3b_b9b9_4685_b56e_974847bc7545);
windows_core::imp::interface_hierarchy!(ICorrelationVectorSource, windows_core::IUnknown);
impl ICorrelationVectorSource {
    pub unsafe fn CorrelationVector(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorrelationVector)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorrelationVectorSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CorrelationVector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICorrelationVectorSource_Impl: windows_core::IUnknownImpl {
    fn CorrelationVector(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl ICorrelationVectorSource_Vtbl {
    pub const fn new<Identity: ICorrelationVectorSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CorrelationVector<Identity: ICorrelationVectorSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorrelationVectorSource_Impl::CorrelationVector(this) {
                    Ok(ok__) => {
                        cv.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CorrelationVector: CorrelationVector::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorrelationVectorSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorrelationVectorSource {}
