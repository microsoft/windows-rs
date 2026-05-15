pub mod Test {
    windows_link::link!("test.dll" "system" fn GetValue() -> u32);
    windows_link::link!("test.dll" "system" fn SetValue(value : u32));
    pub const Blue: Color = 2i32;
    pub type Color = i32;
    #[repr(transparent)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Foo(windows_core::IUnknown);
    windows_core::imp::interface_hierarchy!(
        Foo,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    windows_core::imp::required_hierarchy!(Foo, IFoo2);
    impl Foo {
        pub fn new() -> windows_result::Result<Self> {
            Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
        }
        fn IActivationFactory<
            R,
            F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_result::Result<R>,
        >(
            callback: F,
        ) -> windows_result::Result<R> {
            static SHARED: windows_core::imp::FactoryCache<
                Foo,
                windows_core::imp::IGenericFactory,
            > = windows_core::imp::FactoryCache::new();
            SHARED.call(callback)
        }
        pub fn Stat() -> windows_result::Result<i32> {
            Self::IFooStatics(|this| unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(this).Stat)(
                    windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .map(|| result__)
            })
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
    pub const Green: Color = 1i32;
    pub type HANDLE = *mut core::ffi::c_void;
    windows_core::imp::define_interface!(IFoo, IFoo_Vtbl, 0xf05f601f_33d2_54fc_adca_2df8c56fe621);
    impl windows_core::RuntimeType for IFoo {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    impl IFoo {
        pub fn Direct(&self) -> windows_result::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Direct)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IFoo_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Direct:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
    windows_core::imp::define_interface!(IFoo2, IFoo2_Vtbl, 0xd5639aca_50ae_5b48_9f64_938ce24b8683);
    impl windows_core::RuntimeType for IFoo2 {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IFoo2,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl IFoo2 {
        pub fn Bar(&self) -> windows_result::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Bar)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
    }
    impl windows_core::RuntimeName for IFoo2 {
        const NAME: &'static str = "Test.IFoo2";
    }
    pub trait IFoo2_Impl {
        fn Bar(&self) -> windows_result::Result<i32>;
    }
    impl IFoo2_Vtbl {
        pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> Self
        where
            <Identity as windows_core::IUnknownImpl>::Impl: IFoo2_Impl,
        {
            unsafe extern "system" fn Bar<
                Identity: windows_core::IUnknownImpl,
                const OFFSET: isize,
            >(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT
            where
                <Identity as windows_core::IUnknownImpl>::Impl: IFoo2_Impl,
            {
                unsafe {
                    let this__outer__: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    let this: &<Identity as windows_core::IUnknownImpl>::Impl =
                        <Identity as windows_core::IUnknownImpl>::get_impl(this__outer__);
                    match IFoo2_Impl::Bar(this) {
                        Ok(ok__) => {
                            result__.write(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IFoo2, OFFSET>(),
                Bar: Bar::<Identity, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<IFoo2 as windows_core::Interface>::IID
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IFoo2_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Bar:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
    windows_core::imp::define_interface!(
        IFooStatics,
        IFooStatics_Vtbl,
        0xf1c38ac4_8e1a_5dc8_a1e0_b3b3f956d845
    );
    impl windows_core::RuntimeType for IFooStatics {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IFooStatics_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Stat:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
    pub const INVALID_HANDLE_VALUE: HANDLE = -1i32 as _;
    pub const Red: Color = 0i32;
}
