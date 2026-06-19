#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Class(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Class, windows_core::IUnknown, windows_core::IInspectable);
impl Class {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Class, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(
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
impl windows_core::RuntimeType for Class {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IClass>();
}
unsafe impl windows_core::Interface for Class {
    type Vtable = <IClass as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Class {
    const NAME: &'static str = "Test.Class";
}
windows_core::imp::define_interface!(IClass, IClass_Vtbl, 0x327dfd65_5684_57ee_85ab_168eff31a838);
impl windows_core::RuntimeType for IClass {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.IClass");
}
impl windows_core::RuntimeName for IClass {
    const NAME: &'static str = "Test.IClass";
}
pub trait IClass_Impl: windows_core::IUnknownImpl {
    fn Method(&self) -> windows_core::Result<i32>;
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl IClass_Vtbl {
    pub const fn new<Identity: IClass_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Method<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClass_Impl::Method(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClass_Impl::Name(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClass_Impl::SetName(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IClass, OFFSET>(),
            Method: Method::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClass as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Method:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
