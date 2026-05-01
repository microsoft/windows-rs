#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Foo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Foo,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IFoo
);
windows_core::imp::required_hierarchy!(Foo, IFooFactory);
impl Foo {
    pub fn Hello(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Hello)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    #[cfg(windows)]
    pub fn new() -> windows_core::Result<Foo> {
        Self::IFooFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                &mut core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(windows)]
    pub fn compose<T>(compose: T) -> windows_core::Result<Foo>
    where
        T: windows_core::Compose,
    {
        Self::IFooFactory(|this| unsafe {
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
    #[cfg(windows)]
    fn IFooFactory<R, F: FnOnce(&IFooFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Foo, IFooFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Foo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IFoo>();
}
unsafe impl windows_core::Interface for Foo {
    type Vtable = <IFoo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFoo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Foo {
    const NAME: &'static str = "test_composable_aggregation.Foo";
}
windows_core::imp::define_interface!(IFoo, IFoo_Vtbl, 0x37c80dbe_24c3_5b23_8946_fed1cc4890c9);
impl windows_core::RuntimeType for IFoo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IFoo, windows_core::IUnknown, windows_core::IInspectable);
impl IFoo {
    pub fn Hello(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Hello)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IFoo {
    const NAME: &'static str = "test_composable_aggregation.IFoo";
}
pub trait IFoo_Impl: windows_core::IUnknownImpl {
    fn Hello(&self) -> windows_core::Result<i32>;
}
impl IFoo_Vtbl {
    pub const fn new<Identity: IFoo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Hello<Identity: IFoo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFoo_Impl::Hello(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFoo, OFFSET>(),
            Hello: Hello::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFoo as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFoo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Hello: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IFooFactory,
    IFooFactory_Vtbl,
    0x90084e77_1b8e_5b9b_a4e1_9309956c0cac
);
impl windows_core::RuntimeType for IFooFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IFooFactory,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IFooFactory {
    pub fn CreateInstance<P0>(
        &self,
        outer: P0,
        inner: &mut Option<windows_core::IInspectable>,
    ) -> windows_core::Result<Foo>
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
impl windows_core::RuntimeName for IFooFactory {
    const NAME: &'static str = "test_composable_aggregation.IFooFactory";
}
pub trait IFooFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(
        &self,
        outer: windows_core::Ref<windows_core::IInspectable>,
        inner: windows_core::OutRef<windows_core::IInspectable>,
    ) -> windows_core::Result<Foo>;
}
impl IFooFactory_Vtbl {
    pub const fn new<Identity: IFooFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<
            Identity: IFooFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            outer: *mut core::ffi::c_void,
            inner: *mut *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFooFactory_Impl::CreateInstance(
                    this,
                    core::mem::transmute_copy(&outer),
                    core::mem::transmute_copy(&inner),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFooFactory, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFooFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFooFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
