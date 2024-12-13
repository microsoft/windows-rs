#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

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
    pub fn Signal(&self, value: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Signal)(
                windows_core::Interface::as_raw(this),
                value,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Event<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<windows::Foundation::TypedEventHandler<Class, i32>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Event)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveEvent(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveEvent)(
                windows_core::Interface::as_raw(this),
                token,
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
    const NAME: &'static str = "test_events.Class";
}
unsafe impl Send for Class {}
unsafe impl Sync for Class {}
windows_core::imp::define_interface!(IClass, IClass_Vtbl, 0xad3fd5e5_03a8_5c1e_ab60_efa5e9379730);
impl windows_core::RuntimeType for IClass {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IClass {
    const NAME: &'static str = "test_events.IClass";
}
pub trait IClass_Impl: windows_core::IUnknownImpl {
    fn Signal(&self, value: i32) -> windows_core::Result<i32>;
    fn Event(
        &self,
        handler: windows_core::Ref<'_, windows::Foundation::TypedEventHandler<Class, i32>>,
    ) -> windows_core::Result<i64>;
    fn RemoveEvent(&self, token: i64) -> windows_core::Result<()>;
}
impl IClass_Vtbl {
    pub const fn new<Identity: IClass_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Signal<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: i32,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClass_Impl::Signal(this, value) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Event<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClass_Impl::Event(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveEvent<Identity: IClass_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            token: i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClass_Impl::RemoveEvent(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IClass, OFFSET>(),
            Signal: Signal::<Identity, OFFSET>,
            Event: Event::<Identity, OFFSET>,
            RemoveEvent: RemoveEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClass as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Signal:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Event: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveEvent:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
