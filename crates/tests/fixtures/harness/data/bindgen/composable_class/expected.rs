pub mod Test {
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
        pub fn CreateInstance(name: i32) -> windows_result::Result<Foo> {
            Self::IFooFactory(|this| unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(this).CreateInstance)(
                    windows_core::Interface::as_raw(this),
                    name,
                    core::ptr::null_mut(),
                    &mut core::ptr::null_mut(),
                    &mut result__,
                )
                .and_then(|| windows_core::Type::from_abi(result__))
            })
        }
        pub fn CreateInstance_compose<T>(name: i32, compose: T) -> windows_result::Result<Foo>
        where
            T: windows_core::Compose,
        {
            Self::IFooFactory(|this| unsafe {
                let (derived__, base__) = windows_core::Compose::compose(compose);
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(this).CreateInstance)(
                    windows_core::Interface::as_raw(this),
                    name,
                    core::mem::transmute_copy(&derived__),
                    base__ as *mut _ as _,
                    &mut result__,
                )
                .ok()?;
                let _ = &derived__;
                windows_core::Type::from_abi(result__)
            })
        }
        pub fn new() -> windows_result::Result<Foo> {
            Self::IFooFactory(|this| unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(this).Default)(
                    windows_core::Interface::as_raw(this),
                    core::ptr::null_mut(),
                    &mut core::ptr::null_mut(),
                    &mut result__,
                )
                .and_then(|| windows_core::Type::from_abi(result__))
            })
        }
        pub fn compose<T>(compose: T) -> windows_result::Result<Foo>
        where
            T: windows_core::Compose,
        {
            Self::IFooFactory(|this| unsafe {
                let (derived__, base__) = windows_core::Compose::compose(compose);
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(this).Default)(
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
        fn IFooFactory<R, F: FnOnce(&IFooFactory) -> windows_result::Result<R>>(
            callback: F,
        ) -> windows_result::Result<R> {
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
        const NAME: &'static str = "Test.Foo";
    }
    windows_core::imp::define_interface!(IFoo, IFoo_Vtbl, 0x347b649a_8d93_5e65_aca3_12f0e94dd601);
    impl windows_core::RuntimeType for IFoo {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IFoo,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl windows_core::RuntimeName for IFoo {
        const NAME: &'static str = "Test.IFoo";
    }
    pub trait IFoo_Impl: windows_core::IUnknownImpl {}
    impl IFoo_Vtbl {
        pub const fn new<Identity: IFoo_Impl, const OFFSET: isize>() -> Self {
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IFoo, OFFSET>(),
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
    }
    windows_core::imp::define_interface!(
        IFooFactory,
        IFooFactory_Vtbl,
        0xabcf93a3_b3d8_5966_b4c2_b74691d07150
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
        pub fn CreateInstance<P1>(
            &self,
            name: i32,
            outer: P1,
            inner: &mut Option<windows_core::IInspectable>,
        ) -> windows_result::Result<Foo>
        where
            P1: windows_core::Param<windows_core::IInspectable>,
        {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).CreateInstance)(
                    windows_core::Interface::as_raw(self),
                    name,
                    outer.param().abi(),
                    inner as *mut _ as _,
                    &mut result__,
                )
                .and_then(|| windows_core::Type::from_abi(result__))
            }
        }
        pub fn Default<P0>(
            &self,
            outer: P0,
            inner: &mut Option<windows_core::IInspectable>,
        ) -> windows_result::Result<Foo>
        where
            P0: windows_core::Param<windows_core::IInspectable>,
        {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Default)(
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
        const NAME: &'static str = "Test.IFooFactory";
    }
    pub trait IFooFactory_Impl: windows_core::IUnknownImpl {
        fn CreateInstance(
            &self,
            name: i32,
            outer: windows_core::Ref<windows_core::IInspectable>,
            inner: windows_core::OutRef<windows_core::IInspectable>,
        ) -> windows_result::Result<Foo>;
        fn Default(
            &self,
            outer: windows_core::Ref<windows_core::IInspectable>,
            inner: windows_core::OutRef<windows_core::IInspectable>,
        ) -> windows_result::Result<Foo>;
    }
    impl IFooFactory_Vtbl {
        pub const fn new<Identity: IFooFactory_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn CreateInstance<
                Identity: IFooFactory_Impl,
                const OFFSET: isize,
            >(
                this: *mut core::ffi::c_void,
                name: i32,
                outer: *mut core::ffi::c_void,
                inner: *mut *mut core::ffi::c_void,
                result__: *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IFooFactory_Impl::CreateInstance(
                        this,
                        name,
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
            unsafe extern "system" fn Default<Identity: IFooFactory_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                outer: *mut core::ffi::c_void,
                inner: *mut *mut core::ffi::c_void,
                result__: *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IFooFactory_Impl::Default(
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
                Default: Default::<Identity, OFFSET>,
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
            i32,
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
        ) -> windows_result::HRESULT,
        pub Default: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
        ) -> windows_result::HRESULT,
    }
}
