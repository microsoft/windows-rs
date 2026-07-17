windows_core::imp::define_interface!(ICorrelationVectorInformation, ICorrelationVectorInformation_Vtbl, 0x83c78b3c_d88b_4950_aa6e_22b8d22aabd3);
windows_core::imp::interface_hierarchy!(ICorrelationVectorInformation, windows_core::IUnknown, windows_core::IInspectable);
impl ICorrelationVectorInformation {
    pub unsafe fn LastCorrelationVectorForThread(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastCorrelationVectorForThread)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn NextCorrelationVectorForThread(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NextCorrelationVectorForThread)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetNextCorrelationVectorForThread(&self, cv: &windows_core::HSTRING) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNextCorrelationVectorForThread)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(cv)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorrelationVectorInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LastCorrelationVectorForThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NextCorrelationVectorForThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNextCorrelationVectorForThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICorrelationVectorInformation_Impl: windows_core::IUnknownImpl {
    fn LastCorrelationVectorForThread(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn NextCorrelationVectorForThread(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetNextCorrelationVectorForThread(&self, cv: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl ICorrelationVectorInformation_Vtbl {
    pub const fn new<Identity: ICorrelationVectorInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LastCorrelationVectorForThread<Identity: ICorrelationVectorInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorrelationVectorInformation_Impl::LastCorrelationVectorForThread(this) {
                    Ok(ok__) => {
                        cv.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NextCorrelationVectorForThread<Identity: ICorrelationVectorInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICorrelationVectorInformation_Impl::NextCorrelationVectorForThread(this) {
                    Ok(ok__) => {
                        cv.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNextCorrelationVectorForThread<Identity: ICorrelationVectorInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cv: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICorrelationVectorInformation_Impl::SetNextCorrelationVectorForThread(this, core::mem::transmute(&cv)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICorrelationVectorInformation, OFFSET>(),
            LastCorrelationVectorForThread: LastCorrelationVectorForThread::<Identity, OFFSET>,
            NextCorrelationVectorForThread: NextCorrelationVectorForThread::<Identity, OFFSET>,
            SetNextCorrelationVectorForThread: SetNextCorrelationVectorForThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorrelationVectorInformation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICorrelationVectorInformation {}
