#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Base(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Base,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IOverrides
);
impl Base {
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IBaseFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .ok()?;
            let _ = &derived__;
            windows_core::Type::from_abi(result__)
        })
    }
    fn IBaseFactory<R, F: FnOnce(&IBaseFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Base, IBaseFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Base {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IOverrides>();
}
unsafe impl windows_core::Interface for Base {
    type Vtable = <IOverrides as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOverrides as windows_core::Interface>::IID;
}
impl core::ops::Deref for Base {
    type Target = IOverrides;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Base {
    const NAME: &'static str = "Test.Base";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Derived(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Derived,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IDerived
);
windows_core::imp::required_hierarchy!(Derived, Base);
impl Derived {
    pub fn new() -> windows_core::Result<Self> {
        Self::IDerivedFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IDerivedFactory<R, F: FnOnce(&IDerivedFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Derived, IDerivedFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Derived {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDerived>();
}
unsafe impl windows_core::Interface for Derived {
    type Vtable = <IDerived as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDerived as windows_core::Interface>::IID;
}
impl core::ops::Deref for Derived {
    type Target = IDerived;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Derived {
    const NAME: &'static str = "Test.Derived";
}
windows_core::imp::define_interface!(
    IBaseFactory,
    IBaseFactory_Vtbl,
    0xb2b35536_fcb0_54c1_9424_0abe7fedee40
);
impl windows_core::RuntimeType for IBaseFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IBaseFactory,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IBaseFactory {
    pub fn CreateInstance<P0>(
        &self,
        outer: P0,
        inner: &mut Option<windows_core::IInspectable>,
    ) -> windows_core::Result<Base>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(
                windows_core::Interface::as_raw(self),
                outer.param().abi(),
                inner as *mut _ as _,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IBaseFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDerived,
    IDerived_Vtbl,
    0x23cf7109_3cfc_56a0_88cb_06d8fd320cc6
);
impl windows_core::RuntimeType for IDerived {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IDerived,
    windows_core::IUnknown,
    windows_core::IInspectable
);
#[repr(C)]
pub struct IDerived_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IDerivedFactory,
    IDerivedFactory_Vtbl,
    0xa04ab497_ffe5_53d3_a6ad_1bfb9db8c9a1
);
impl windows_core::RuntimeType for IDerivedFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IDerivedFactory,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IDerivedFactory {
    pub fn CreateInstance<P0>(
        &self,
        outer: P0,
        inner: &mut Option<windows_core::IInspectable>,
    ) -> windows_core::Result<Derived>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(
                windows_core::Interface::as_raw(self),
                outer.param().abi(),
                inner as *mut _ as _,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDerivedFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IOverrides,
    IOverrides_Vtbl,
    0x8952afe3_c320_51da_92db_224e61ab57d1
);
impl windows_core::RuntimeType for IOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.IOverrides");
}
windows_core::imp::interface_hierarchy!(
    IOverrides,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for IOverrides {
    const NAME: &'static str = "Test.IOverrides";
}
pub trait IOverrides_Impl: windows_core::IUnknownImpl {
    fn OnSomething(&self) -> windows_core::Result<()>;
}
impl IOverrides_Vtbl {
    pub const fn new<Identity: IOverrides_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSomething<Identity: IOverrides_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOverrides_Impl::OnSomething(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IOverrides, OFFSET>(),
            OnSomething: OnSomething::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOverrides as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnSomething: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
