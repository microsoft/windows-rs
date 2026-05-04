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
    windows_core::imp::required_hierarchy!(Foo, IFooFactory, IFooStatics);
    impl Foo {
        pub fn CreateInstance(name: i32) -> windows_result::Result<Foo> {
            Self::IFooFactory(|this| unsafe {
                windows_core::imp::call_in_out(this, |this__, result__| {
                    (windows_core::Interface::vtable(this).CreateInstance)(
                        this__,
                        name,
                        core::ptr::null_mut(),
                        &mut core::ptr::null_mut(),
                        result__,
                    )
                })
            })
        }
        pub fn CreateInstance_compose<T>(name: i32, compose: T) -> windows_result::Result<Foo>
        where
            T: windows_core::Compose,
        {
            Self::IFooFactory(|this| unsafe {
                windows_core::imp::call_compose(
                    this,
                    compose,
                    |this__, derived__, base__, result__| {
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            this__, name, derived__, base__, result__,
                        )
                    },
                )
            })
        }
        pub fn Reset() -> windows_result::Result<()> {
            Self::IFooStatics(|this| unsafe {
                windows_core::imp::call_in(this, |this__| {
                    (windows_core::Interface::vtable(this).Reset)(this__)
                })
            })
        }
        pub fn Current() -> windows_result::Result<Foo> {
            Self::IFooStatics(|this| unsafe {
                windows_core::imp::call_in_out(this, |this__, result__| {
                    (windows_core::Interface::vtable(this).Current)(this__, result__)
                })
            })
        }
        fn IFooFactory<R, F: FnOnce(&IFooFactory) -> windows_result::Result<R>>(
            callback: F,
        ) -> windows_result::Result<R> {
            static SHARED: windows_core::imp::FactoryCache<Foo, IFooFactory> =
                windows_core::imp::FactoryCache::new();
            SHARED.call(callback)
        }
        fn IFooStatics<R, F: FnOnce(&IFooStatics) -> windows_result::Result<R>>(
            callback: F,
        ) -> windows_result::Result<R> {
            static SHARED: windows_core::imp::FactoryCache<Foo, IFooStatics> =
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
        0xcf8e99c0_2712_5467_ac7d_89f9097933fd
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
                windows_core::imp::call_in_out(self, |this__, result__| {
                    (windows_core::Interface::vtable(self).CreateInstance)(
                        this__,
                        name,
                        outer.param().abi(),
                        inner as *mut _ as _,
                        result__,
                    )
                })
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
            i32,
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
        ) -> windows_result::HRESULT,
    }
    windows_core::imp::define_interface!(
        IFooStatics,
        IFooStatics_Vtbl,
        0x55a42e35_71fe_548e_9d28_6573a98f27c8
    );
    impl windows_core::RuntimeType for IFooStatics {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IFooStatics,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl IFooStatics {
        pub fn Reset(&self) -> windows_result::Result<()> {
            unsafe {
                windows_core::imp::call_in(self, |this__| {
                    (windows_core::Interface::vtable(self).Reset)(this__)
                })
            }
        }
        pub fn Current(&self) -> windows_result::Result<Foo> {
            unsafe {
                windows_core::imp::call_in_out(self, |this__, result__| {
                    (windows_core::Interface::vtable(self).Current)(this__, result__)
                })
            }
        }
    }
    impl windows_core::RuntimeName for IFooStatics {
        const NAME: &'static str = "Test.IFooStatics";
    }
    pub trait IFooStatics_Impl: windows_core::IUnknownImpl {
        fn Reset(&self) -> windows_result::Result<()>;
        fn Current(&self) -> windows_result::Result<Foo>;
    }
    impl IFooStatics_Vtbl {
        pub const fn new<Identity: IFooStatics_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn Reset<Identity: IFooStatics_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    IFooStatics_Impl::Reset(this).into()
                }
            }
            unsafe extern "system" fn Current<Identity: IFooStatics_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IFooStatics_Impl::Current(this) {
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
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IFooStatics, OFFSET>(),
                Reset: Reset::<Identity, OFFSET>,
                Current: Current::<Identity, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<IFooStatics as windows_core::Interface>::IID
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IFooStatics_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_result::HRESULT,
        pub Current: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
        ) -> windows_result::HRESULT,
    }
    windows_core::imp::define_interface!(ITest, ITest_Vtbl, 0x28337665_44f1_5895_b1c3_726675c39971);
    impl windows_core::RuntimeType for ITest {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        ITest,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl ITest {
        pub fn DoSomething(&self) -> windows_result::Result<()> {
            unsafe {
                windows_core::imp::call_in(self, |this__| {
                    (windows_core::Interface::vtable(self).DoSomething)(this__)
                })
            }
        }
        pub fn GetValue(&self) -> windows_result::Result<i32> {
            unsafe {
                windows_core::imp::call_in_out(self, |this__, result__| {
                    (windows_core::Interface::vtable(self).GetValue)(this__, result__)
                })
            }
        }
        pub fn GetThing(&self) -> windows_result::Result<ITest> {
            unsafe {
                windows_core::imp::call_in_out(self, |this__, result__| {
                    (windows_core::Interface::vtable(self).GetThing)(this__, result__)
                })
            }
        }
    }
    impl windows_core::RuntimeName for ITest {
        const NAME: &'static str = "Test.ITest";
    }
    pub trait ITest_Impl: windows_core::IUnknownImpl {
        fn DoSomething(&self) -> windows_result::Result<()>;
        fn GetValue(&self) -> windows_result::Result<i32>;
        fn GetThing(&self) -> windows_result::Result<ITest>;
    }
    impl ITest_Vtbl {
        pub const fn new<Identity: ITest_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn DoSomething<Identity: ITest_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    ITest_Impl::DoSomething(this).into()
                }
            }
            unsafe extern "system" fn GetValue<Identity: ITest_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match ITest_Impl::GetValue(this) {
                        Ok(ok__) => {
                            result__.write(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            unsafe extern "system" fn GetThing<Identity: ITest_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match ITest_Impl::GetThing(this) {
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
                base__: windows_core::IInspectable_Vtbl::new::<Identity, ITest, OFFSET>(),
                DoSomething: DoSomething::<Identity, OFFSET>,
                GetValue: GetValue::<Identity, OFFSET>,
                GetThing: GetThing::<Identity, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<ITest as windows_core::Interface>::IID
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct ITest_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub DoSomething:
            unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_result::HRESULT,
        pub GetValue:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
        pub GetThing: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
        ) -> windows_result::HRESULT,
    }
}
