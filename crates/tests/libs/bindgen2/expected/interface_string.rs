windows_core::imp::define_interface!(
    Interface,
    Interface_Vtbl,
    0xf7039fad_567f_5194_8150_4a7613a4b64c
);
impl windows_core::RuntimeType for Interface {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.Interface");
}
windows_core::imp::interface_hierarchy!(
    Interface,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Interface {
    pub fn GetName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetName)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for Interface {
    const NAME: &'static str = "Test.Interface";
}
pub trait Interface_Impl: windows_core::IUnknownImpl {
    fn GetName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl Interface_Vtbl {
    pub const fn new<Identity: Interface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: Interface_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Interface_Impl::GetName(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: Interface_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Interface_Impl::SetName(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, Interface, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Interface as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct Interface_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
