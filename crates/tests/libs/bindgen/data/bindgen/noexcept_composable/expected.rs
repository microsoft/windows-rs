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
        pub fn CreateInstance(name: i32) -> Option<Foo> {
            Self::IFooFactory(|this| {
                Ok(unsafe {
                    let mut result__ = core::mem::zeroed();
                    let hresult__ = (windows_core::Interface::vtable(this).CreateInstance)(
                        windows_core::Interface::as_raw(this),
                        name,
                        core::ptr::null_mut(),
                        &mut core::ptr::null_mut(),
                        &mut result__,
                    );
                    debug_assert!(hresult__.0 == 0);
                    core::mem::transmute(result__)
                })
            })
            .unwrap()
        }
        pub fn CreateInstance_compose<T>(name: i32, compose: T) -> Option<Foo>
        where
            T: windows_core::Compose,
        {
            Self::IFooFactory(|this| {
                Ok(unsafe {
                    let (derived__, base__) = windows_core::Compose::compose(compose);
                    let mut result__ = core::mem::zeroed();
                    let hresult__ = (windows_core::Interface::vtable(this).CreateInstance)(
                        windows_core::Interface::as_raw(this),
                        name,
                        core::mem::transmute_copy(&derived__),
                        base__ as *mut _ as _,
                        &mut result__,
                    );
                    debug_assert!(hresult__.0 == 0);
                    let _ = &derived__;
                    core::mem::transmute(result__)
                })
            })
            .unwrap()
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
}
