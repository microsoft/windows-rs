windows_core::imp::define_interface!(IBase, IBase_Vtbl, 0x4c195fa3_ba8e_5836_826e_7fde3bb2dad9);
impl windows_core::RuntimeType for IBase {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.IBase");
}
windows_core::imp::interface_hierarchy!(IBase, windows_core::IUnknown, windows_core::IInspectable);
impl IBase {
    pub fn First(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).First)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IBase {
    const NAME: &'static str = "Test.IBase";
}
pub trait IBase_Impl: windows_core::IUnknownImpl {
    fn First(&self) -> windows_core::Result<i32>;
}
impl IBase_Vtbl {
    pub const fn new<Identity: IBase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn First<Identity: IBase_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBase_Impl::First(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBase, OFFSET>(),
            First: First::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBase as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub First: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDerived,
    IDerived_Vtbl,
    0xe55b3cfe_28ca_5296_a10d_b0d7cb570517
);
impl windows_core::RuntimeType for IDerived {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.IDerived");
}
windows_core::imp::interface_hierarchy!(
    IDerived,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(IDerived, IBase);
impl IDerived {
    pub fn Second(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Second)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IDerived {
    const NAME: &'static str = "Test.IDerived";
}
pub trait IDerived_Impl: IBase_Impl {
    fn Second(&self) -> windows_core::Result<i32>;
}
impl IDerived_Vtbl {
    pub const fn new<Identity: IDerived_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Second<Identity: IDerived_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDerived_Impl::Second(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDerived, OFFSET>(),
            Second: Second::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDerived as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IDerived_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Second:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
