#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IInstance,
    IInstance_Vtbl,
    0x4cc554b9_8483_54a9_8490_1467dfd7078f
);
impl windows_core::RuntimeType for IInstance {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IInstance {
    const NAME: &'static str = "test_activation.One.IInstance";
}
pub trait IInstance_Impl: windows_core::IUnknownImpl {
    fn Property(&self) -> windows_core::Result<i32>;
}
impl IInstance_Vtbl {
    pub const fn new<Identity: IInstance_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Property<Identity: IInstance_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstance_Impl::Property(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInstance, OFFSET>(),
            Property: Property::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstance as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstance_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Property:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMissing,
    IMissing_Vtbl,
    0xad54a92f_16de_537c_b6c0_5099534ee12e
);
impl windows_core::RuntimeType for IMissing {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IMissing {
    const NAME: &'static str = "test_activation.One.IMissing";
}
pub trait IMissing_Impl: windows_core::IUnknownImpl {
    fn Method(&self) -> windows_core::Result<()>;
}
impl IMissing_Vtbl {
    pub const fn new<Identity: IMissing_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Method<Identity: IMissing_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMissing_Impl::Method(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMissing, OFFSET>(),
            Method: Method::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMissing as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMissing_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Method: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IStaticStatics,
    IStaticStatics_Vtbl,
    0x530ccab2_1b46_5dba_a8bb_a857df3dc803
);
impl windows_core::RuntimeType for IStaticStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for IStaticStatics {
    const NAME: &'static str = "test_activation.One.Two.Three.Four.IStaticStatics";
}
pub trait IStaticStatics_Impl: windows_core::IUnknownImpl {
    fn Property(&self) -> windows_core::Result<i32>;
}
impl IStaticStatics_Vtbl {
    pub const fn new<Identity: IStaticStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Property<Identity: IStaticStatics_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStaticStatics_Impl::Property(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStaticStatics, OFFSET>(),
            Property: Property::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStaticStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStaticStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Property:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instance(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Instance,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Instance {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            Instance,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Property(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Property)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for Instance {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IInstance>();
}
unsafe impl windows_core::Interface for Instance {
    type Vtable = <IInstance as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInstance as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Instance {
    const NAME: &'static str = "test_activation.One.Instance";
}
unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Missing(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Missing,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Missing {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            Missing,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Method(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Method)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
impl windows_core::RuntimeType for Missing {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IMissing>();
}
unsafe impl windows_core::Interface for Missing {
    type Vtable = <IMissing as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMissing as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Missing {
    const NAME: &'static str = "test_activation.One.Missing";
}
unsafe impl Send for Missing {}
unsafe impl Sync for Missing {}
pub struct Static;
impl Static {
    pub fn Property() -> windows_core::Result<i32> {
        Self::IStaticStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Property)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    fn IStaticStatics<R, F: FnOnce(&IStaticStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Static, IStaticStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for Static {
    const NAME: &'static str = "test_activation.One.Two.Three.Four.Static";
}
