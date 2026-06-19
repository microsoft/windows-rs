windows_core::imp::define_interface!(
    Interface,
    Interface_Vtbl,
    0x69fb0d53_69de_5906_95b6_8dc296141466
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
}
impl windows_core::RuntimeName for Interface {
    const NAME: &'static str = "Test.Interface";
}
pub trait Interface_Impl: windows_core::IUnknownImpl {
    fn Execute(&self) -> windows_core::Result<()>;
    fn Method(&self) -> windows_core::Result<i32>;
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
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, Interface, OFFSET>(),
            Execute: Execute::<Identity, OFFSET>,
            Method: Method::<Identity, OFFSET>,
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
}
