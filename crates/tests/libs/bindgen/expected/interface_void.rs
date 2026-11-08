windows_core::imp::define_winrt_interface!(
    Interface,
    Interface_Vtbl,
    0xe42fee02_e196_5756_865b_aa4392a97a64,
    b"Test.Interface"
);
windows_core::imp::interface_hierarchy!(
    Interface,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Interface {
    pub fn Execute(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub fn Method(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Method)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Text)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Values(&self) -> windows_core::Result<windows_core::Array<u8>> {
        unsafe {
            let mut result__ = core::ptr::null_mut();
            let mut result_size__ = 0;
            (windows_core::Interface::vtable(self).Values)(
                windows_core::Interface::as_raw(self),
                &mut result_size__,
                &mut result__,
            )
            .map(|| windows_core::Array::<u8>::from_raw_parts(result__.cast(), result_size__))
        }
    }
    pub fn Partial(&self, value: &mut u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Partial)(
                windows_core::Interface::as_raw(self),
                value,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for Interface {
    const NAME: &'static str = "Test.Interface";
}
pub trait Interface_Impl: windows_core::IUnknownImpl {
    fn Execute(&self) -> windows_core::Result<()>;
    fn Method(&self) -> windows_core::Result<i32>;
    fn Text(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Values(&self) -> windows_core::Result<windows_core::Array<u8>>;
    fn Partial(&self, value: &mut u32) -> windows_core::Result<u32>;
}
impl Interface_Vtbl {
    pub const fn new<Identity: Interface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Execute<Identity: Interface_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Interface_Impl::Execute(this).into()
            }
        }
        unsafe extern "system" fn Method<Identity: Interface_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Interface_Impl::Method(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        result__.write_bytes(0, 1);
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn Text<Identity: Interface_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Interface_Impl::Text(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        result__.write_bytes(0, 1);
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn Values<Identity: Interface_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut u8,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Interface_Impl::Values(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(ok_data__);
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        result__.write_bytes(0, 1);
                        result_size__.write(0);
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn Partial<Identity: Interface_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut u32,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Interface_Impl::Partial(this, core::mem::transmute_copy(&value)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        result__.write_bytes(0, 1);
                        err.into()
                    }
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, OFFSET>(),
            Execute: Execute::<Identity, OFFSET>,
            Method: Method::<Identity, OFFSET>,
            Text: Text::<Identity, OFFSET>,
            Values: Values::<Identity, OFFSET>,
            Partial: Partial::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Interface as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct Interface_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Method:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Values: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut u8,
    ) -> windows_core::HRESULT,
    pub Partial: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut u32,
    ) -> windows_core::HRESULT,
}
