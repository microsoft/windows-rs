pub mod Test {
    #[repr(transparent)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Foo(windows_core::IUnknown);
    windows_core::imp::interface_hierarchy!(
        Foo,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl Foo {
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
        pub fn KeepFactory() -> windows_result::Result<Foo> {
            Self::IFooFactory(|this| unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(this).KeepFactory)(
                    windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .and_then(|| windows_core::Type::from_abi(result__))
            })
        }
        pub fn KeepStatic() -> windows_result::Result<i32> {
            Self::IFooStatics(|this| unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(this).KeepStatic)(
                    windows_core::Interface::as_raw(this),
                    &mut result__,
                )
                .map(|| result__)
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
    windows_core::imp::define_interface!(IFoo, IFoo_Vtbl, 0xf05f601f_33d2_54fc_adca_2df8c56fe621);
    impl windows_core::RuntimeType for IFoo {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
        const NAME: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::from_slice(b"Test.IFoo");
    }
    #[repr(C)]
    pub struct IFoo_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Direct:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
    windows_core::imp::define_interface!(
        IFooFactory,
        IFooFactory_Vtbl,
        0xf2539086_0371_5fe0_be9a_60e2098fd12d
    );
    impl windows_core::RuntimeType for IFooFactory {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
        const NAME: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::from_slice(b"Test.IFooFactory");
    }
    #[repr(C)]
    pub struct IFooFactory_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub KeepFactory: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
        ) -> windows_result::HRESULT,
    }
    windows_core::imp::define_interface!(
        IFooStatics,
        IFooStatics_Vtbl,
        0xba941a54_1f25_5c77_870d_65a7a7b6a7c8
    );
    impl windows_core::RuntimeType for IFooStatics {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
        const NAME: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::from_slice(b"Test.IFooStatics");
    }
    #[repr(C)]
    pub struct IFooStatics_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub KeepStatic:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
}
